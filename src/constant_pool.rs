pub const CONST_CLASS: u8 = 7;
pub const CONST_FIELDREF: u8 = 9;
pub const CONST_METHODREF: u8 = 10;
pub const CONST_INTERFACE_METHODREF: u8 = 11;
pub const CONST_STRING: u8 = 8;
pub const CONST_INTEGER: u8 = 3;
pub const CONST_FLOAT: u8 = 4;
pub const CONST_LONG: u8 = 5;
pub const CONST_DOUBLE: u8 = 6;
pub const CONST_NAME_AND_TYPE: u8 = 12;
pub const CONST_UTF8: u8 = 1;
pub const CONST_METHOD_HANDLE: u8 = 15;
pub const CONST_METHOD_TYPE: u8 = 16;
pub const CONST_INVOKE_DYNAMIC: u8 = 18;

#[derive(Debug)]
pub(crate) enum Constant<'c> {
    Class {
        tag: u8,
        name_index: u16
    },
    Fieldref {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16
    },
    Methodref {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16
    },
    InterfaceMethodref {
        tag: u8,
        class_index: u16,
        name_and_type_index: u16
    },
    String {
        tag: u8,
        string_index: u16
    },
    Integer {
        tag: u8,
        bytes: [u8; 4]
    },
    Float {
        tag: u8,
        bytes: [u8; 4]
    },
    Utf8 {
        tag: u8,
        length: u16,
        bytes: &'c [u8]
    },
    NameAndType {
        tag: u8,
        name_index: u16,
        descriptor_index: u16
    }
    
}
