use crate::access_flags::FieldAccessFlag;
use crate::attribute::Attribute;

/// Describes a field <br>
/// See <https://docs.oracle.com/javase/specs/jvms/se14/html/jvms-4.html#jvms-4.5> for more information
#[derive(Debug)]
pub struct Field {
    pub access_flags: FieldAccessFlag,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>
}
