pub mod frame;

use crate::gen::{java::code::CodeContext, GenerateCode, GeneratorBackend};
use jvm_class_format::{
    attribute::{AttributeValue, ExceptionTableEntry},
    Constant, Instruction, Op,
};
use std::collections::HashMap;

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
    fn new(base: &'b RuntimeBase, ctx: CodeContext<'c>) -> Self {
        RuntimeFrame {
            base,

            exception_table: ctx.exception_table,
            attributes: ctx.attributes,

            stack_size: ctx.max_stack,
            stack: Vec::with_capacity(ctx.max_stack),

            max_locals: ctx.max_locals,
        }
    }

    pub fn push_to_stack(&mut self, value: StackValue) {
        if self.stack_size < self.stack.len() {
            tracing::warn!("exceeded stack limit!");
        }

        self.stack.push(value);
    }
}

pub struct OpSeq<const LENGTH: usize>(pub [Op; LENGTH]);

impl<const L: usize> OpSeq<L> {
    pub fn test(&self, buffer: impl AsRef<[Instruction]>, offset: usize) -> bool {
        if L > buffer.as_ref()[offset..].len() {
            return false;
        }
        for (i, op) in self.0.iter().enumerate() {
            if buffer.as_ref()[offset + i].op != *op {
                return false;
            }
        }

        return true;
    }
}

pub struct SeqVariants<const COUNT: usize, const LENGTH: usize>(pub [OpSeq<LENGTH>; COUNT]);

// pg. 620 - 15.12.4. Run-Time Evaluation of Method Invocation

macro_rules! test_many_expr {
    (&[$first: ty $(,$e: ty) *], $instructions: expr, $offset: expr) => {
        <$first>::test($instructions, $offset)$(
            .or_else(|| <$e>::test($instructions, $offset)))*
    };
}

pub fn to_ir<'a, 'r, 'c>(
    lang: &'r RuntimeBase,
    ctx: CodeContext<'c>,
    code: impl AsRef<[u8]>,
) -> Vec<Expression> {
    let instructions = Instruction::from_bytecode(code);

    let frame = RuntimeFrame::new(lang, ctx);

    let mut result = Vec::with_capacity(instructions.len());

    let mut offset = 0;
    while offset < instructions.len() {
        #[rustfmt::skip]
        let (instruction_count, expr) = test_many_expr!(&[
            EmptySuperCall,
            InstructionComment
        ], &instructions, offset).unwrap();
        offset += instruction_count;
        result.push(expr);
    }
    debug_assert_eq!(
        offset,
        instructions.len(),
        "to_ir overshot instruction buffer"
    );

    result.shrink_to_fit();
    result
}

pub trait TestExpression {
    fn test(buffer: impl AsRef<[Instruction]>, offset: usize) -> Option<(usize, Expression)>;
}

pub enum Expression {
    Super(EmptySuperCall),
    Comment(InstructionComment),
}

#[derive(Debug)]
pub struct InstructionComment(pub Instruction);

impl TestExpression for InstructionComment {
    fn test(instr: impl AsRef<[Instruction]>, offset: usize) -> Option<(usize, Expression)> {
        unsafe {
            Some((
                1,
                Expression::Comment(InstructionComment(
                    instr.as_ref().get_unchecked(offset).clone(),
                )),
            ))
        }
    }
}

pub struct EmptySuperCall;

impl TestExpression for EmptySuperCall {
    fn test(buffer: impl AsRef<[Instruction]>, offset: usize) -> Option<(usize, Expression)> {
        let result = OpSeq([
            Op::Aload0, // push this to stack
            Op::Invokespecial,
        ])
        .test(buffer, offset);

        if !result {
            return None;
        }

        Some((2, Expression::Super(Self)))
    }
}
