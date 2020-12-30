
pub(crate) struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

pub(crate) enum Attribute<'c> {
    ConstantValue {
        attribute_name_index: u16,
        attribute_length: u16,
        constantvalue_index: u16
    },
    Code {
        attribute_name_index: u16,
        attribute_length: u16,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: &'c [u8],
        exception_table_length: u16,
        exception_table: Vec<ExceptionTableEntry>,
        attributes_count: u16,
        attributes: Vec<Attribute<'c>>
    }
}
