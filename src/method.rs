use crate::access_flags::MethodAccessFlag;
use crate::attribute::Attribute;

/// Describes a method <br>
/// See <https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.6> for more information
#[derive(Debug)]
pub struct Method {
    pub access_flags: MethodAccessFlag,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>
}
