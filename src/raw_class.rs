use crate::constant_pool::{
    CONST_METHODREF,
    CONST_FIELDREF,
    CONST_STRING,
    CONST_CLASS,
    CONST_UTF8,
    CONST_NAME_AND_TYPE
};
use crate::constant_pool::Constant;

#[derive(Debug)]
pub struct RawClass {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,

    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interface_count: u16
}

impl RawClass {
    pub fn from_bytes(bytes: &[u8]) -> RawClass {
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let minor_version = u16::from_be_bytes([bytes[4], bytes[5]]);
        let major_version = u16::from_be_bytes([bytes[6], bytes[7]]);
        let constant_pool_count = u16::from_be_bytes([bytes[8], bytes[9]]);

        let mut i = 1;
        let mut offset = 10;

        while i < constant_pool_count {

            let tag = bytes[offset];

            let constant = match tag {
                CONST_METHODREF => {
                    let class_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let name_and_type_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::Methodref { tag, class_index, name_and_type_index }
                },
                CONST_FIELDREF => {
                    let class_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let name_and_type_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::Fieldref { tag, class_index, name_and_type_index }
                },
                CONST_STRING => {
                    let string_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    offset += 3;

                    Constant::String { tag, string_index }
                },
                CONST_CLASS => {
                    let name_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    offset += 3;

                    Constant::Class { tag, name_index }
                },
                CONST_UTF8 => {
                    let length = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let end = offset + 3 + length as usize;
                    let bytes = &bytes[offset + 3 .. end];
                    offset += length as usize + 3;

                    Constant::Utf8 { tag, length, bytes }
                },
                CONST_NAME_AND_TYPE => {
                    let name_index = u16::from_be_bytes([bytes[offset + 1], bytes[offset + 2]]);
                    let descriptor_index = u16::from_be_bytes([bytes[offset + 3], bytes[offset + 4]]);
                    offset += 5;

                    Constant::NameAndType { tag, name_index, descriptor_index }
                }
                _ => panic!("unknown constant tag {}", tag)
            };

            println!("{:#?}", constant);
            i += 1;
        }

        let access_flags = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        let this_class = u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]);
        let super_class = u16::from_be_bytes([bytes[offset + 4], bytes[offset + 5]]);
        let interface_count = u16::from_be_bytes([bytes[offset + 6], bytes[offset + 7]]);

        RawClass {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            access_flags,
            this_class,
            super_class,
            interface_count
        }
    }
}
