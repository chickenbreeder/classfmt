use crate::access_flags::MethodAccessFlag;
use crate::attribute::Attribute;

#[derive(Debug)]
pub struct Method {
    pub access_flags: MethodAccessFlag,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>
}
