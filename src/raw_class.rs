use std::str;
use std::convert::TryFrom;

use crate::constant_pool::{ConstantTag, Constant, ReferenceKind};
use crate::attribute::Attribute;
use crate::error::ErrorType;
use crate::field::Field;
use crate::method::Method;

#[derive(Debug)]
pub struct RawClass<'c> {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<Constant<'c>>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interface_count: u16,
    pub field_count: u16,
    pub fields: Vec<Field<'c>>,
    pub methods_count: u16,
    pub methods: Vec<Method<'c>>
}

impl<'c> RawClass<'c> {
    pub fn from_bytes(bytes: &'c [u8]) -> Result<RawClass<'c>, ErrorType> {
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let minor_version = u16::from_be_bytes([bytes[4], bytes[5]]);
        let major_version = u16::from_be_bytes([bytes[6], bytes[7]]);
        let constant_pool_count = u16::from_be_bytes([bytes[8], bytes[9]]);

        let offset = 10;
        let (constant_pool, mut offset) = Self::read_constant_pool(bytes, offset, constant_pool_count)?;

        let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        let this_class = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
        let super_class = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
        let interface_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);
        let field_count = u16::from_be_bytes([bytes[offset + 8], bytes[offset + 9]]);
        offset += 10;

        let (fields, mut offset) = Self::read_fields(bytes, offset, field_count, &constant_pool)?;

        let methods_count = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let (methods, _) = Self::read_methods(bytes, offset, methods_count, &constant_pool)?;

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
            methods
        })
    }

    fn read_constant_pool(bytes: &'c [u8], offset: usize, constant_pool_count: u16) -> Result<(Vec<Constant<'c>>, usize), ErrorType> {
        let mut offset = offset;
        let mut i = 1;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);

        while i < constant_pool_count {
            let tag = ConstantTag::try_from(bytes[offset])?;

            let constant = match tag {
                ConstantTag::Methodref => {
                    let class_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let name_and_type_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::Methodref { tag, class_index, name_and_type_index }
                },
                ConstantTag::Fieldref => {
                    let class_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let name_and_type_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::Fieldref { tag, class_index, name_and_type_index }
                },
                ConstantTag::String => {
                    let string_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    offset += 3;

                    Constant::String { tag, string_index }
                },
                ConstantTag::Class => {
                    let name_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    offset += 3;

                    Constant::Class { tag, name_index }
                },
                ConstantTag::Utf8 => {
                    let length = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let end = offset + 3 + length as usize;
                    let bytes = &bytes[offset + 3 .. end];
                    offset += length as usize + 3;

                    Constant::Utf8 { tag, length, bytes }
                },
                ConstantTag::NameAndType => {
                    let name_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let descriptor_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::NameAndType { tag, name_index, descriptor_index }
                },
                ConstantTag::Integer => {
                    
                    let b = [bytes[offset + 1], bytes[offset + 2], bytes[offset + 3], bytes[offset + 4]];
                    let value = i32::from_be_bytes(b);
                    offset += 5;

                    Constant::Integer { tag, value }
                },
                ConstantTag::MethodHandle => {
                    let reference_kind = ReferenceKind::try_from(bytes[offset + 1])?;
                    let reference_index = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
                    offset += 4;

                    Constant::MethodHandle { tag, reference_kind, reference_index }
                },
                ConstantTag::InvokeDynamic => {
                    let bootstrap_method_attr_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let name_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

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

        Ok((constant_pool, offset))
    }

    fn read_fields(bytes: &[u8], offset: usize, field_count: u16, constant_pool: &[Constant]) -> Result<(Vec<Field<'c>>, usize), ErrorType> {
        let mut offset = offset;
        let mut i = 0;
        let mut fields = Vec::with_capacity(field_count as usize);

        while i < field_count {
            let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let name_index = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            let descriptor_index = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
            let attributes_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);
            offset += 8;

            let (attributes, new_offset) = Self::read_attributes(bytes, offset, attributes_count, constant_pool)?;
            offset = new_offset;

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

        Ok((fields, offset))
    }

    fn read_methods(bytes: &[u8], offset: usize, method_count: u16, constant_pool: &[Constant]) -> Result<(Vec<Method<'c>>, usize), ErrorType> {
        let mut offset = offset;
        let mut i = 0;
        let mut methods = Vec::with_capacity(method_count as usize);
        
        while i < method_count {
            let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let name_index = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            let descriptor_index = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
            let attributes_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);
            offset += 8;

            let (attributes, new_offset) = Self::read_attributes(bytes, offset, attributes_count, constant_pool)?;
            offset = new_offset;

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

        Ok((methods, offset))
    }

    fn read_attributes(bytes: &[u8], offset: usize, attribute_count: u16, constant_pool: &[Constant]) -> Result<(Vec<Attribute<'c>>, usize), ErrorType> {
        let mut offset = offset;
        let mut i = 0;
        let mut attributes = Vec::with_capacity(attribute_count as usize);

        while i < attribute_count {
            let attribute_name_index = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let attribute_length = u32::from_be_bytes([bytes[offset + 2], bytes[offset + 3], bytes[offset + 4], bytes[offset + 5]]);
            offset += 6;

            let cp_entry = &constant_pool[(attribute_name_index - 1) as usize];

            match cp_entry {
                &Constant::Utf8 {tag, length, bytes: s_bytes} => {
                    let s = str::from_utf8(s_bytes)?;

                    let attribute = match s {
                        "ConstantValue" => {
                            let constantvalue_index = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
                            offset += 2;

                            Attribute::ConstantValue {
                                attribute_name_index,
                                attribute_length,
                                constantvalue_index
                            }
                        },
                        "Code" => {
                            let max_stack = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
                            let max_locals = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
                            let code_length = u32::from_be_bytes([bytes[offset + 4], bytes[offset + 5], bytes[offset + 6], bytes[offset + 7]]);
                            offset += 8;

                            Self::read_code(bytes, offset, code_length);
                            unimplemented!()
                        },
                        _ => panic!("unknown tag: `{}`", s)
                    };

                    attributes.push(attribute);
                },
                _ => return Err(ErrorType::InvalidNameIndex)
            }

            i += 1;
        }

        Ok((attributes, offset))
    }

    fn read_code(bytes: &[u8], offset: usize, code_length: u32) {
        
    }
}

#[cfg(test)]
mod test {
    use std::{
        str,
        fs::File,
        path::Path,
        io,
        io::Read
    };
    use super::RawClass;
    use crate::error::ErrorType;
    use crate::attribute::Attribute;
    use crate::constant_pool::{Constant, ConstantTag};

    fn read_class_file(p: &str) -> Result<Vec<u8>, ErrorType> {
        let mut f = File::open(Path::new(p)).unwrap();
        let mut buf = Vec::with_capacity(64);
    
        f.read_to_end(&mut buf).unwrap();
        Ok(buf)
    }

    #[test]
    fn parse_fields() {
        let buf = read_class_file("./tests/Fields.class").unwrap();
        let class = RawClass::from_bytes(&buf).unwrap();

        assert_eq!(class.field_count, 3);
        let constant_pool = class.constant_pool;
        let fields = class.fields;
        let f0 = &fields[0];

        let constant = &constant_pool[(f0.name_index - 1) as usize];

        if let Constant::Utf8 { tag, length, bytes } = constant {
            let s = str::from_utf8(bytes).unwrap();

            assert_eq!(s, "test");
            assert_eq!(f0.attributes_count, 1);
            let attribute = &f0.attributes[0];

            if let Attribute::ConstantValue { attribute_name_index, attribute_length, constantvalue_index } = attribute {
                let constant = &constant_pool[(constantvalue_index - 1) as usize];

                if let Constant::Integer {tag, value} = constant {
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
