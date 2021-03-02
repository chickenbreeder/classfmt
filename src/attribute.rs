use crate::opcode::Opcode;

#[derive(Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue {
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16
    },
    Code {
        attribute_name_index: u16,
        attribute_length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<Opcode>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTableEntry>,
        attributes_count: u16,
        attributes: Vec<Attribute>
    },
    LineNumberTable {
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table_length: u16,
        line_number_table: Vec<LineNumberTableEntry>
    }
}
