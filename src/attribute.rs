use crate::access_flags::{InnerClassAccessFlag, ParameterAccessFlag};
use crate::opcode::Instruction;

#[derive(Debug)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16
}

#[derive(Debug)]
pub struct LineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16
}

#[derive(Debug)]
pub struct InnerClassAttribute {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: InnerClassAccessFlag
}

#[derive(Debug)]
pub struct ParameterAttribute {
    pub name_index: u16,
    pub access_flags: ParameterAccessFlag
}

#[derive(Debug)]
pub struct BootstrapMethodAttribute {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>
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
        code: Vec<Instruction>,
        exception_table_length: u16,
        exception_table: Vec<ExceptionTableEntry>,
        attributes_count: u16,
        attributes: Vec<Attribute>
    },
    InnerClasses {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        classes: Vec<InnerClassAttribute>
    },
    SourceFile {
        attribute_name_index: u16,
        attribute_length: u32,
        sourcefile_index: u16
    },
    LineNumberTable {
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table_length: u16,
        line_number_table: Vec<LineNumberTableEntry>
    },
    BootstrapMethods {
        attribute_name_index: u16,
        attribute_length: u32,
        num_bootstrap_methods: u16,
        bootstrap_methods: Vec<BootstrapMethodAttribute>
    },
    MethodParameters {
        attribute_name_index: u16,
        attribute_length: u32,
        parameters_count: u8,
        parameters: Vec<ParameterAttribute>
    },
    NestMembers {
        attribute_name_index: u16,
        attribute_length: u32,
        number_of_classes: u16,
        classes: Vec<u16>
    }
}
