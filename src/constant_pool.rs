use std::convert::TryFrom;
use crate::error::ErrorType;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum ConstantTag {
    Class = 7,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    String = 8,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    NameAndType = 12,
    Utf8 = 1,
    MethodHandle = 15,
    MethodType = 16,
    InvokeDynamic = 18
}

impl TryFrom<u8> for ConstantTag {
    type Error = ErrorType;

    fn try_from(v: u8) -> Result<ConstantTag, Self::Error> {
        match v {
            x if x == ConstantTag::Class as u8 => Ok(ConstantTag::Class),
            x if x == ConstantTag::Fieldref as u8 => Ok(ConstantTag::Fieldref),
            x if x == ConstantTag::Methodref as u8 => Ok(ConstantTag::Methodref),
            x if x == ConstantTag::InterfaceMethodref as u8 => Ok(ConstantTag::InterfaceMethodref),
            x if x == ConstantTag::String as u8 => Ok(ConstantTag::String),
            x if x == ConstantTag::Integer as u8 => Ok(ConstantTag::Integer),
            x if x == ConstantTag::Float as u8 => Ok(ConstantTag::Float),
            x if x == ConstantTag::Long as u8 => Ok(ConstantTag::Long),
            x if x == ConstantTag::Double as u8 => Ok(ConstantTag::Double),
            x if x == ConstantTag::NameAndType as u8 => Ok(ConstantTag::NameAndType),
            x if x == ConstantTag::Utf8 as u8 => Ok(ConstantTag::Utf8),
            x if x == ConstantTag::MethodHandle as u8 => Ok(ConstantTag::MethodHandle),
            x if x == ConstantTag::MethodType as u8 => Ok(ConstantTag::MethodType),
            x if x == ConstantTag::InvokeDynamic as u8 => Ok(ConstantTag::InvokeDynamic),
            _ => Err(ErrorType::IntegerConversion)
        }
    }
}

#[derive(Debug)]
pub(crate) enum Constant<'c> {
    Class {
        tag: ConstantTag,
        name_index: u16
    },
    Fieldref {
        tag: ConstantTag,
        class_index: u16,
        name_and_type_index: u16
    },
    Methodref {
        tag: ConstantTag,
        class_index: u16,
        name_and_type_index: u16
    },
    InterfaceMethodref {
        tag: ConstantTag,
        class_index: u16,
        name_and_type_index: u16
    },
    String {
        tag: ConstantTag,
        string_index: u16
    },
    Integer {
        tag: ConstantTag,
        value: i32
    },
    Float {
        tag: ConstantTag,
        bytes: &'c [u8]
    },
    Utf8 {
        tag: ConstantTag,
        length: u16,
        bytes: &'c [u8]
    },
    NameAndType {
        tag: ConstantTag,
        name_index: u16,
        descriptor_index: u16
    }
    
}
