use std::str;
use std::convert::TryFrom;

use crate::constant_pool::{ConstantTag, Constant};
use crate::attributes::Attribute;
use crate::error::ErrorType;

#[derive(Debug)]
pub struct RawClass<'c> {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<Constant<'c>>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interface_count: u16,
    // interfaces: Vec<u16>, <--- index into the constant pool table,
    field_count: u16
}

struct Field<'c> {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute<'c>>
}

impl<'c> RawClass<'c> {
    pub fn from_bytes(bytes: &'c [u8]) -> Result<RawClass<'c>, ErrorType> {
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let minor_version = u16::from_be_bytes([bytes[4], bytes[5]]);
        let major_version = u16::from_be_bytes([bytes[6], bytes[7]]);
        let constant_pool_count = u16::from_be_bytes([bytes[8], bytes[9]]);

        let mut i = 1;
        let mut offset = 10;

        let (constant_pool, mut offset) = Self::read_constant_pool(bytes, offset, constant_pool_count)?;

        let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        let this_class = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
        let super_class = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
        let interface_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);

        // TODO: if interface_count > 0, read interfaces

        let field_count = u16::from_be_bytes([bytes[offset + 8], bytes[offset + 9]]);
        offset += 10;

        let _ = Self::read_fields(bytes, offset, field_count, &constant_pool)?;

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
            field_count
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
                }
                _ => panic!("unknown constant tag {}", tag as u8)
            };

            constant_pool.push(constant);
            i += 1;
        }

        Ok((constant_pool, offset))
    }

    fn read_fields(bytes: &[u8], offset: usize, field_count: u16, constant_pool: &[Constant]) -> Result<usize, ErrorType> {
        let mut offset = offset;
        let mut i = 0;

        while i < field_count {
            let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
            let name_index = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
            let descriptor_index = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
            let attributes_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);
            offset += 8;

            let (attributes, new_offset) = Self::read_attributes(bytes, offset, attributes_count, constant_pool)?;
            offset = new_offset;

            i += 1;
        }

        Ok(offset)
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
                        _ => unimplemented!()
                    };

                    attributes.push(attribute);
                },
                _ => return Err(ErrorType::InvalidNameIndex)
            }

            i += 1;
        }

        Ok((attributes, offset))
    }
}
