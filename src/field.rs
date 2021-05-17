use crate::access_flags::FieldAccessFlag;
use crate::attribute::Attribute;

#[derive(Debug)]
pub struct Field {
    pub access_flags: FieldAccessFlag,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>
}
