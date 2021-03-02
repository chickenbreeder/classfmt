use std::convert::TryFrom;
use std::str;

use crate::error::ErrorType;
use crate::{Constant, ConstantTag, ReferenceKind, Attribute, Opcode, Method, Field, RawClass};
use crate::attribute::{LineNumberTableEntry, ExceptionTableEntry};

pub struct ClassParser<'c> {
    bytes: &'c [u8],
    offset: usize
}

impl<'c> ClassParser<'c> {
    pub fn from_bytes(bytes: &'c [u8]) -> ClassParser<'c> {
        ClassParser {
            bytes,
            offset: 0
        }
    }

    pub fn parse(&mut self) -> Result<RawClass, ErrorType> {
        let magic = self.read_u32_be();
        let minor_version = self.read_u16_be();
        let major_version = self.read_u16_be();
        let constant_pool_count = self.read_u16_be();
        let constant_pool = self.read_constant_pool(constant_pool_count)?;
        let access_flags = self.read_u16_be();
        let this_class = self.read_u16_be();
        let super_class = self.read_u16_be();
        let interface_count = self.read_u16_be();
        let field_count = self.read_u16_be();
        let fields = self.read_fields(field_count, &constant_pool)?;
        let methods_count = self.read_u16_be();
        let methods = self.read_methods(methods_count, &constant_pool)?;

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
        })
    }

    fn read_u32_be(&mut self) -> u32 {
        let off = self.offset;
        self.offset += 4;
        u32::from_be_bytes([self.bytes[off], self.bytes[off + 1], self.bytes[off + 2], self.bytes[off + 3]])
    }

    fn read_i32_be(&mut self) -> i32 {
        let off = self.offset;
        self.offset += 4;
        i32::from_be_bytes([self.bytes[off], self.bytes[off + 1], self.bytes[off + 2], self.bytes[off + 3]])
    }

    fn read_u16_be(&mut self) -> u16 {
        let off = self.offset;
        self.offset += 2;
        u16::from_be_bytes([self.bytes[off], self.bytes[off + 1]])
    }

    fn read_constant_pool(&mut self, constant_pool_count: u16) -> Result<Vec<Constant<'c>>, ErrorType> {
        let mut i = 1;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);

        while i < constant_pool_count {
            let tag = ConstantTag::try_from(self.bytes[self.offset])?;
            self.offset += 1;

            let constant = match tag {
                ConstantTag::Methodref => {
                    let class_index = self.read_u16_be();
                    let name_and_type_index = self.read_u16_be();

                    Constant::Methodref { tag, class_index, name_and_type_index }
                },
                ConstantTag::Fieldref => {
                    let class_index = self.read_u16_be();
                    let name_and_type_index = self.read_u16_be();

                    Constant::Fieldref { tag, class_index, name_and_type_index }
                },
                ConstantTag::String => {
                    let string_index = self.read_u16_be();

                    Constant::String { tag, string_index }
                },
                ConstantTag::Class => {
                    let name_index = self.read_u16_be();

                    Constant::Class { tag, name_index }
                },
                ConstantTag::Utf8 => {
                    let length = self.read_u16_be();
                    let end = self.offset + length as usize;
                    let bytes = &self.bytes[self.offset .. end];
                    self.offset += length as usize;

                    Constant::Utf8 { tag, length, bytes }
                },
                ConstantTag::NameAndType => {
                    let name_index = self.read_u16_be();
                    let descriptor_index = self.read_u16_be();

                    Constant::NameAndType { tag, name_index, descriptor_index }
                },
                ConstantTag::Integer => {
                    let value = self.read_i32_be();

                    Constant::Integer { tag, value }
                },
                ConstantTag::MethodHandle => {
                    let reference_kind = ReferenceKind::try_from(self.bytes[self.offset])?;
                    self.offset += 1;
                    let reference_index = self.read_u16_be();

                    Constant::MethodHandle { tag, reference_kind, reference_index }
                },
                ConstantTag::InvokeDynamic => {
                    let bootstrap_method_attr_index = self.read_u16_be();
                    let name_index = self.read_u16_be();

                    Constant::InvokeDynamic {
                        tag,
                        bootstrap_method_attr_index,
                        name_index
                    }
                },
                _ => panic!("unknown constant tag {}", tag as u8)
            };

            constant_pool.push(constant);
            i += 1;
        }

        Ok(constant_pool)
    }

    fn read_fields(&mut self, field_count: u16, constant_pool: &[Constant]) -> Result<Vec<Field>, ErrorType> {
        let mut i = 0;
        let mut fields = Vec::with_capacity(field_count as usize);

        while i < field_count {
            let access_flags = self.read_u16_be();
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

    fn read_methods(&mut self, method_count: u16, constant_pool: &[Constant]) -> Result<Vec<Method>, ErrorType> {
        let mut i = 0;
        let mut methods = Vec::with_capacity(method_count as usize);
        
        while i < method_count {
            let access_flags = self.read_u16_be();
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

    fn read_attributes(&mut self, attribute_count: u16, constant_pool: &[Constant]) -> Result<Vec<Attribute>, ErrorType> {
        let mut i = 0;
        let mut attributes = Vec::with_capacity(attribute_count as usize);

        while i < attribute_count {
            let attribute_name_index = self.read_u16_be();
            let attribute_length = self.read_u32_be();

            let cp_entry = &constant_pool[(attribute_name_index - 1) as usize];

            match cp_entry {
                &Constant::Utf8 {tag: _, length: _, bytes: s_bytes} => {
                    let s = str::from_utf8(s_bytes)?;

                    let attribute = match s {
                        "ConstantValue" => {
                            let constantvalue_index = self.read_u16_be();

                            Attribute::ConstantValue {
                                attribute_name_index,
                                attribute_length,
                                constantvalue_index
                            }
                        },
                        "Code" => self.read_code_attribute(attribute_name_index, attribute_length, constant_pool)?,
                        "LineNumberTable" => self.read_line_number_table_attribute(attribute_name_index, attribute_length)?,
                        _ => panic!("unknown tag: `{}`", s)
                    };

                    attributes.push(attribute);
                },
                _ => return Err(ErrorType::InvalidNameIndex)
            }

            i += 1;
        }

        Ok(attributes)
    }

    fn read_code_attribute(&mut self, attribute_name_index: u16, attribute_length: u32, constant_pool: &[Constant]) -> Result<Attribute, ErrorType> {
        let max_stack = self.read_u16_be();
        let max_locals = self.read_u16_be();
        let code_length = self.read_u32_be();
        let code = self.read_opcodes(code_length)?;

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

    fn read_line_number_table_attribute(&mut self, attribute_name_index: u16, attribute_length: u32) -> Result<Attribute, ErrorType> {
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
            line_number_table,
        })
    }

    fn read_opcodes(&mut self, code_length: u32) -> Result<Vec<Opcode>, ErrorType> {
        let mut i = 0;
        let mut opcodes = Vec::with_capacity(code_length as usize);

        while i < code_length {
            let opcode = Opcode::try_from(self.bytes[self.offset])?;
            self.offset += 1;
            opcodes.push(opcode);

            i += 1;
        }

        Ok(opcodes)
    }
}

#[cfg(test)]
mod test {
    use std::{
        str,
        fs::File,
        path::Path,
        io::Read
    };
    use super::ClassParser;
    use crate::error::ErrorType;
    use crate::attribute::Attribute;
    use crate::constant_pool::Constant;

    fn read_class_file(p: &str) -> Result<Vec<u8>, ErrorType> {
        let mut f = File::open(Path::new(p)).unwrap();
        let mut buf = Vec::with_capacity(64);
    
        f.read_to_end(&mut buf).unwrap();
        Ok(buf)
    }

    #[test]
    fn parse_fields() {
        let buf = read_class_file("./tests/Fields.class").unwrap();
        let mut parser = ClassParser::from_bytes(&buf);
        let class = parser.parse().unwrap();

        assert_eq!(class.field_count, 3);
        let constant_pool = class.constant_pool;
        let fields = class.fields;
        let f0 = &fields[0];

        let constant = &constant_pool[(f0.name_index - 1) as usize];

        if let Constant::Utf8 { tag: _, length: _, bytes } = constant {
            let s = str::from_utf8(bytes).unwrap();

            assert_eq!(s, "test");
            assert_eq!(f0.attributes_count, 1);
            let attribute = &f0.attributes[0];

            if let Attribute::ConstantValue { attribute_name_index: _, attribute_length: _, constantvalue_index } = attribute {
                let constant = &constant_pool[(constantvalue_index - 1) as usize];

                if let Constant::Integer {tag: _, value} = constant {
                    assert_eq!(*value, 2147483647);
                }
                else {
                    panic!("expected Constant::Integer, found {:?}", constant);
                }
            }
            else {
                panic!("expected Attribute::ConstantValue, found {:?}", attribute);
            }
        }
        else {
            panic!("expected Constant::Utf8, found {:?}", constant);
        }
    }
}
