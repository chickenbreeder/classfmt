use crate::{Constant, Field, Method};

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
    pub fields: Vec<Field>,
    pub methods_count: u16,
    pub methods: Vec<Method>
}
