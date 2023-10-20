use std::collections::HashMap;

use jvm_class_format::{
    attribute::{AttributeValue, ExceptionTableEntry, CodeData},
    ConstantPool,
};

pub struct RuntimeBase {
    pub constant_pool: ConstantPool,
}

#[derive(Debug, Clone, Copy)]
pub enum StackValue {}

#[derive(Debug, Clone)]
pub struct RuntimeFrame<'cp, 'code> {
    pub constant_pool: &'cp ConstantPool,

    pub exception_table: &'code [ExceptionTableEntry],
    pub attributes: &'code HashMap<String, AttributeValue>,

    pub stack_size: usize,
    pub stack: Vec<StackValue>,

    pub max_locals: usize,
}

impl<'cp, 'code> RuntimeFrame<'cp, 'code> {
    pub fn new(base: &'cp ConstantPool, code: &'code CodeData) -> Self {
        RuntimeFrame {
            constant_pool: base,

            exception_table: &code.exception_table,
            attributes: &code.attributes,

            stack_size: code.max_stack,
            stack: Vec::with_capacity(code.max_stack),

            max_locals: code.max_locals,
        }
    }

    pub fn new_inner(&self) -> Self {
        RuntimeFrame {
            constant_pool: self.constant_pool,

            exception_table: self.exception_table,
            attributes: self.attributes,

            stack_size: self.stack_size,
            stack: self.stack.clone(),

            max_locals: self.max_locals,
        }
    }

    pub fn push_to_stack(&mut self, value: StackValue) {
        if self.stack_size < self.stack.len() {
            tracing::warn!("exceeded stack limit!");
        }

        self.stack.push(value);
    }
}
