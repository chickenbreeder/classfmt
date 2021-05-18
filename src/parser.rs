use std::convert::TryFrom;
use std::str;

use crate::attribute::{
    BootstrapMethodAttribute, ExceptionTableEntry, InnerClassAttribute, LineNumberTableEntry,
    ParameterAttribute
};
use crate::error::ErrorType;
use crate::{Attribute, Constant, ConstantTag, Field, Method, Opcode, RawClass, ReferenceKind};

use crate::access_flags::{
    ClassAccessFlag, FieldAccessFlag, InnerClassAccessFlag, MethodAccessFlag, ParameterAccessFlag
};
use crate::opcode::Instruction;

/// The class parser. Used to construct instances of [`RawClass`]
pub struct ClassParser<'c> {
    bytes: &'c [u8],
    offset: usize
}

impl<'c> ClassParser<'c> {
    /// Creates a new parser from given bytes
    pub fn from_bytes(bytes: &'c [u8]) -> ClassParser<'c> {
        ClassParser { bytes, offset: 0 }
    }

    /// Parses the provided bytes and tries to construct a new instance of [`RawClass`]
    pub fn parse(&mut self) -> Result<RawClass<'c>, ErrorType> {
        let magic = self.read_u32_be();
        let minor_version = self.read_u16_be();
        let major_version = self.read_u16_be();
        let constant_pool_count = self.read_u16_be();
        let constant_pool = self.read_constant_pool(constant_pool_count)?;
        let access_flags = ClassAccessFlag::from_bits(self.read_u16_be()).unwrap();
        let this_class = self.read_u16_be();
        let super_class = self.read_u16_be();
        let interface_count = self.read_u16_be();
        let field_count = self.read_u16_be();
        let fields = self.read_fields(field_count, &constant_pool)?;
        let methods_count = self.read_u16_be();
        let methods = self.read_methods(methods_count, &constant_pool)?;
        let attributes_count = self.read_u16_be();
        let attributes = self.read_attributes(attributes_count, &constant_pool)?;

        Ok(RawClass {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interface_count,
            field_count,
            fields,
            methods_count,
            methods,
            attributes_count,
            attributes
        })
    }

    fn read_u32_be(&mut self) -> u32 {
        let off = self.offset;
        self.offset += 4;
        u32::from_be_bytes([
            self.bytes[off],
            self.bytes[off + 1],
            self.bytes[off + 2],
            self.bytes[off + 3]
        ])
    }

    fn read_i32_be(&mut self) -> i32 {
        let off = self.offset;
        self.offset += 4;
        i32::from_be_bytes([
            self.bytes[off],
            self.bytes[off + 1],
            self.bytes[off + 2],
            self.bytes[off + 3]
        ])
    }

    fn read_u16_be(&mut self) -> u16 {
        let off = self.offset;
        self.offset += 2;
        u16::from_be_bytes([self.bytes[off], self.bytes[off + 1]])
    }

    fn read_constant_pool(
        &mut self,
        constant_pool_count: u16
    ) -> Result<Vec<Constant<'c>>, ErrorType> {
        let mut i = 1;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);

        while i < constant_pool_count {
            let tag = ConstantTag::try_from(self.bytes[self.offset])?;
            self.offset += 1;

            let constant = match tag {
                ConstantTag::Methodref => {
                    let class_index = self.read_u16_be();
                    let name_and_type_index = self.read_u16_be();

                    Constant::Methodref {
                        tag,
                        class_index,
                        name_and_type_index
                    }
                }
                ConstantTag::Fieldref => {
                    let class_index = self.read_u16_be();
                    let name_and_type_index = self.read_u16_be();

                    Constant::Fieldref {
                        tag,
                        class_index,
                        name_and_type_index
                    }
                }
                ConstantTag::String => {
                    let string_index = self.read_u16_be();

                    Constant::String { tag, string_index }
                }
                ConstantTag::Class => {
                    let name_index = self.read_u16_be();

                    Constant::Class { tag, name_index }
                }
                ConstantTag::Utf8 => {
                    let length = self.read_u16_be();
                    let end = self.offset + length as usize;
                    let bytes = &self.bytes[self.offset..end];
                    self.offset += length as usize;

                    Constant::Utf8 { tag, length, bytes }
                }
                ConstantTag::NameAndType => {
                    let name_index = self.read_u16_be();
                    let descriptor_index = self.read_u16_be();

                    Constant::NameAndType {
                        tag,
                        name_index,
                        descriptor_index
                    }
                }
                ConstantTag::Integer => {
                    let value = self.read_i32_be();

                    Constant::Integer { tag, value }
                }
                ConstantTag::MethodHandle => {
                    let reference_kind = ReferenceKind::try_from(self.bytes[self.offset])?;
                    self.offset += 1;
                    let reference_index = self.read_u16_be();

                    Constant::MethodHandle {
                        tag,
                        reference_kind,
                        reference_index
                    }
                }
                ConstantTag::InvokeDynamic => {
                    let bootstrap_method_attr_index = self.read_u16_be();
                    let name_index = self.read_u16_be();

                    Constant::InvokeDynamic {
                        tag,
                        bootstrap_method_attr_index,
                        name_index
                    }
                }
                _ => unimplemented!("Unsupported constant tag {:?}", tag)
            };

            constant_pool.push(constant);
            i += 1;
        }

        Ok(constant_pool)
    }

    fn read_fields(
        &mut self,
        field_count: u16,
        constant_pool: &[Constant]
    ) -> Result<Vec<Field>, ErrorType> {
        let mut i = 0;
        let mut fields = Vec::with_capacity(field_count as usize);

        while i < field_count {
            let access_flags = FieldAccessFlag::from_bits(self.read_u16_be()).unwrap();
            let name_index = self.read_u16_be();
            let descriptor_index = self.read_u16_be();
            let attributes_count = self.read_u16_be();

            let attributes = self.read_attributes(attributes_count, constant_pool)?;

            let field = Field {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes
            };

            fields.push(field);

            i += 1;
        }

        Ok(fields)
    }

    fn read_methods(
        &mut self,
        method_count: u16,
        constant_pool: &[Constant]
    ) -> Result<Vec<Method>, ErrorType> {
        let mut i = 0;
        let mut methods = Vec::with_capacity(method_count as usize);

        while i < method_count {
            let access_flags = MethodAccessFlag::from_bits(self.read_u16_be()).unwrap();
            let name_index = self.read_u16_be();
            let descriptor_index = self.read_u16_be();
            let attributes_count = self.read_u16_be();

            let attributes = self.read_attributes(attributes_count, constant_pool)?;

            let method = Method {
                access_flags,
                name_index,
                descriptor_index,
                attributes_count,
                attributes
            };

            methods.push(method);

            i += 1;
        }

        Ok(methods)
    }

    fn read_attributes(
        &mut self,
        attribute_count: u16,
        constant_pool: &[Constant]
    ) -> Result<Vec<Attribute>, ErrorType> {
        let mut i = 0;
        let mut attributes = Vec::with_capacity(attribute_count as usize);

        while i < attribute_count {
            let attribute_name_index = self.read_u16_be();
            let attribute_length = self.read_u32_be();

            if let Constant::Utf8 {
                tag: _,
                length: _,
                bytes
            } = &constant_pool[(attribute_name_index - 1) as usize]
            {
                let s = str::from_utf8(bytes)?;

                let attribute = match s {
                    "ConstantValue" => {
                        let constantvalue_index = self.read_u16_be();

                        Attribute::ConstantValue {
                            attribute_name_index,
                            attribute_length,
                            constantvalue_index
                        }
                    }
                    "Code" => self.read_code_attribute(
                        attribute_name_index,
                        attribute_length,
                        constant_pool
                    )?,
                    "InnerClasses" => {
                        let number_of_classes = self.read_u16_be();
                        let classes = self.read_inner_class_attributes(number_of_classes);

                        Attribute::InnerClasses {
                            attribute_name_index,
                            attribute_length,
                            number_of_classes,
                            classes
                        }
                    }
                    "LineNumberTable" => self
                        .read_line_number_table_attribute(attribute_name_index, attribute_length)?,
                    "SourceFile" => {
                        let sourcefile_index = self.read_u16_be();

                        Attribute::SourceFile {
                            attribute_name_index,
                            attribute_length,
                            sourcefile_index
                        }
                    }
                    "BootstrapMethods" => {
                        let num_bootstrap_methods = self.read_u16_be();
                        let bootstrap_methods =
                            self.read_bootstrap_method_attributes(num_bootstrap_methods);

                        Attribute::BootstrapMethods {
                            attribute_name_index,
                            attribute_length,
                            num_bootstrap_methods,
                            bootstrap_methods
                        }
                    }
                    "MethodParameters" => {
                        let parameters_count = self.bytes[self.offset];
                        self.offset += 1;
                        let parameters = self.read_method_parameter_attributes(parameters_count);

                        Attribute::MethodParameters {
                            attribute_name_index,
                            attribute_length,
                            parameters_count,
                            parameters
                        }
                    }
                    "NestMembers" => {
                        let number_of_classes = self.read_u16_be();
                        let mut classes = Vec::with_capacity(number_of_classes as usize);

                        let mut i = 0;
                        while i < number_of_classes {
                            classes.push(self.read_u16_be());
                            i += 1;
                        }

                        Attribute::NestMembers {
                            attribute_name_index,
                            attribute_length,
                            number_of_classes,
                            classes
                        }
                    }
                    _ => panic!("unknown tag: `{}`", s)
                };

                attributes.push(attribute);
            } else {
                return Err(ErrorType::InvalidNameIndex);
            }

            i += 1;
        }

        Ok(attributes)
    }

    fn read_code_attribute(
        &mut self,
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &[Constant]
    ) -> Result<Attribute, ErrorType> {
        let max_stack = self.read_u16_be();
        let max_locals = self.read_u16_be();
        let code_length = self.read_u32_be();
        let (offset, code) = self.read_instructions(code_length)?;
        self.offset = offset;

        let exception_table_length = self.read_u16_be();
        let mut exception_table = Vec::with_capacity(exception_table_length as usize);
        let mut i = 0;

        while i < exception_table_length {
            let start_pc = self.read_u16_be();
            let end_pc = self.read_u16_be();
            let handler_pc = self.read_u16_be();
            let catch_type = self.read_u16_be();

            let entry = ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type
            };

            exception_table.push(entry);
            i += 1;
        }

        let attributes_count = self.read_u16_be();
        let attributes = self.read_attributes(attributes_count, constant_pool)?;

        Ok(Attribute::Code {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes
        })
    }

    fn read_inner_class_attributes(&mut self, number_of_classes: u16) -> Vec<InnerClassAttribute> {
        let mut classes = Vec::with_capacity(number_of_classes as usize);
        let mut i = 0;

        while i < number_of_classes {
            let inner_class_info_index = self.read_u16_be();
            let outer_class_info_index = self.read_u16_be();
            let inner_name_index = self.read_u16_be();
            let inner_class_access_flags =
                InnerClassAccessFlag::from_bits(self.read_u16_be()).unwrap();

            classes.push(InnerClassAttribute {
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags
            });
            i += 1;
        }

        classes
    }

    fn read_method_parameter_attributes(
        &mut self,
        parameters_count: u8
    ) -> Vec<ParameterAttribute> {
        let mut parameters = Vec::with_capacity(parameters_count as usize);
        let mut i = 0;

        while i < parameters_count {
            let name_index = self.read_u16_be();
            let access_flags = ParameterAccessFlag::from_bits(self.read_u16_be()).unwrap();

            parameters.push(ParameterAttribute {
                name_index,
                access_flags
            });
            i += 1;
        }

        parameters
    }

    fn read_bootstrap_method_attributes(
        &mut self,
        num_bootstrap_methods: u16
    ) -> Vec<BootstrapMethodAttribute> {
        let mut methods = Vec::with_capacity(num_bootstrap_methods as usize);
        let mut i = 0;

        while i < num_bootstrap_methods {
            let bootstrap_method_ref = self.read_u16_be();
            let num_bootstrap_arguments = self.read_u16_be();
            let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
            let mut j = 0;

            while j < num_bootstrap_arguments {
                bootstrap_arguments.push(self.read_u16_be());
                j += 1;
            }

            methods.push(BootstrapMethodAttribute {
                bootstrap_method_ref,
                num_bootstrap_arguments,
                bootstrap_arguments
            });

            i += 1;
        }

        methods
    }

    fn read_line_number_table_attribute(
        &mut self,
        attribute_name_index: u16,
        attribute_length: u32
    ) -> Result<Attribute, ErrorType> {
        let line_number_table_length = self.read_u16_be();
        let mut i = 0;
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);

        while i < line_number_table_length {
            let start_pc = self.read_u16_be();
            let line_number = self.read_u16_be();

            let entry = LineNumberTableEntry {
                start_pc,
                line_number
            };

            line_number_table.push(entry);
            i += 1;
        }

        Ok(Attribute::LineNumberTable {
            attribute_name_index,
            attribute_length,
            line_number_table_length,
            line_number_table
        })
    }

    fn read_instructions(
        &mut self,
        code_length: u32
    ) -> Result<(usize, Vec<Instruction>), ErrorType> {
        let mut offset = self.offset;
        let mut instructions = Vec::with_capacity(code_length as usize);

        while offset < self.offset + code_length as usize {
            let opcode = Opcode::try_from(self.bytes[offset])?;
            offset += 1;

            let ins = match opcode {
                Opcode::aload_0 => Instruction::aload_0,
                Opcode::aload_1 => Instruction::aload_1,
                Opcode::aload_2 => Instruction::aload_2,
                Opcode::aload_3 => Instruction::aload_3,
                Opcode::astore => {
                    let index = self.bytes[offset];
                    offset += 1;

                    Instruction::astore { index }
                }
                Opcode::astore_0 => Instruction::astore_0,
                Opcode::astore_1 => Instruction::astore_1,
                Opcode::astore_2 => Instruction::astore_2,
                Opcode::astore_3 => Instruction::astore_3,
                Opcode::dup => Instruction::dup,
                Opcode::bipush => {
                    let byte = self.bytes[offset];
                    offset += 1;

                    Instruction::bipush { byte }
                }
                Opcode::new => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    offset += 2;

                    Instruction::new {
                        indexbyte1,
                        indexbyte2
                    }
                }
                Opcode::r#eturn => Instruction::r#eturn,
                Opcode::invokedynamic => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    let byte3 = self.bytes[offset + 2];
                    let byte4 = self.bytes[offset + 3];
                    offset += 4;

                    if byte3 != 0 || byte4 != 0 {
                        return Err(ErrorType::ParseError);
                    }

                    Instruction::invokedynamic {
                        indexbyte1,
                        indexbyte2,
                        byte3,
                        byte4
                    }
                }
                Opcode::invokestatic => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    offset += 2;

                    Instruction::invokestatic {
                        indexbyte1,
                        indexbyte2
                    }
                }
                Opcode::invokespecial => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    offset += 2;

                    Instruction::invokespecial {
                        indexbyte1,
                        indexbyte2
                    }
                }
                Opcode::invokevirtual => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    offset += 2;

                    Instruction::invokevirtual {
                        indexbyte1,
                        indexbyte2
                    }
                }
                Opcode::getstatic => {
                    let indexbyte1 = self.bytes[offset];
                    let indexbyte2 = self.bytes[offset + 1];
                    offset += 2;

                    Instruction::getstatic {
                        indexbyte1,
                        indexbyte2
                    }
                }
                Opcode::ldc => {
                    let index = self.bytes[offset];
                    offset += 1;

                    Instruction::ldc { index }
                }
                _ => unimplemented!("Unsupported opcode {:?}", opcode)
            };

            instructions.push(ins);
        }

        Ok((offset, instructions))
    }
}

#[cfg(test)]
mod test {
    macro_rules! expect_pat {
        ($expected: pat, $expr: expr, $then: block) => {
            if let $expected = $expr $then
            else {
                panic!("Expected {:?}, found {:?}", stringify!($expected), $expr);
            }
        };
    }

    use super::ClassParser;
    use crate::attribute::Attribute;
    use crate::constant_pool::Constant;
    use crate::error::ErrorType;
    use std::{fs::File, io::Read, path::Path, str};

    fn read_class_file(p: &str) -> Result<Vec<u8>, ErrorType> {
        let mut f = File::open(Path::new(p)).unwrap();
        let mut buf = Vec::with_capacity(64);

        f.read_to_end(&mut buf).unwrap();
        Ok(buf)
    }

    #[test]
    fn parse_simple() {
        let buf = read_class_file("./tests/Hello.class").unwrap();
        let class = ClassParser::from_bytes(&buf)
            .parse()
            .unwrap();

        assert_eq!(class.magic, 0xCAFEBABE);
        assert_eq!(class.methods_count, 2);
        assert_eq!(class.field_count, 0);

        let constant_pool = class.constant_pool;

        expect_pat!(Constant::Class {tag: _, name_index}, &constant_pool[(class.this_class - 1) as usize], {
            expect_pat!(Constant::Utf8 {tag: _, length: _, bytes}, &constant_pool[(name_index - 1) as usize], {
                assert_eq!(str::from_utf8(bytes).unwrap(), "Hello");
            });
        });

        expect_pat!(Constant::Class {tag: _, name_index}, &constant_pool[(class.super_class - 1) as usize], {
            expect_pat!(Constant::Utf8 {tag: _, length: _, bytes}, &constant_pool[(name_index - 1) as usize], {
                assert_eq!(str::from_utf8(bytes).unwrap(), "java/lang/Object");
            });
        });
    }

    #[test]
    fn parse_fields() {
        let buf = read_class_file("./tests/Fields.class").unwrap();
        let class = ClassParser::from_bytes(&buf)
            .parse()
            .unwrap();

        assert_eq!(class.field_count, 3);
        let constant_pool = class.constant_pool;
        let fields = class.fields;
        let f0 = &fields[0];

        let constant = &constant_pool[(f0.name_index - 1) as usize];

        expect_pat!(Constant::Utf8{tag: _, length: _, bytes}, constant, {
            assert_eq!(str::from_utf8(bytes).unwrap(), "test");
            assert_eq!(f0.attributes_count, 1);
            let attribute = &f0.attributes[0];

            expect_pat!(Attribute::ConstantValue{attribute_name_index: _, attribute_length: _, constantvalue_index}, attribute, {
                expect_pat!(Constant::Integer{tag: _, value}, &constant_pool[(constantvalue_index - 1) as usize], {
                    assert_eq!(*value, 2147483647);
                });
            });
        });
    }
}
