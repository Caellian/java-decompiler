use std::collections::HashMap;

use jvm_class_format::{
    attribute::{AttributeValue, ExceptionTableEntry},
    Constant,
};

use crate::gen::java::code::MethodContext;

pub struct RuntimeBase {
    pub constant_pool: HashMap<u16, Constant>,
}

pub enum StackValue {}

pub struct RuntimeFrame<'b, 'c> {
    pub base: &'b RuntimeBase,

    pub exception_table: &'c [ExceptionTableEntry],
    pub attributes: &'c HashMap<String, AttributeValue>,

    pub stack_size: usize,
    pub stack: Vec<StackValue>,

    pub max_locals: usize,
}

impl<'b, 'c> RuntimeFrame<'b, 'c> {
    pub fn new(base: &'b RuntimeBase, ctx: MethodContext<'c>) -> Self {
        RuntimeFrame {
            base,

            exception_table: &ctx.code.exception_table,
            attributes: &ctx.code.attributes,

            stack_size: ctx.code.max_stack,
            stack: Vec::with_capacity(ctx.code.max_stack),

            max_locals: ctx.code.max_locals,
        }
    }

    pub fn push_to_stack(&mut self, value: StackValue) {
        if self.stack_size < self.stack.len() {
            tracing::warn!("exceeded stack limit!");
        }

        self.stack.push(value);
    }
}
