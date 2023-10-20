use jvm_class_format::{attribute::CodeData, Instruction, Op};

use super::frame::RuntimeFrame;

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

#[macro_export]
macro_rules! test_many_expr {
    (&[$first: ty $(,$other: ty) *], $instructions: expr, $offset: expr, $ctx: expr) => {
        <$first>::test($instructions, $offset, $ctx)$(
            .or_else(|| <$other>::test($instructions, $offset, $ctx)))*
    };
}

pub trait CheckExpression {
    fn test<'cp, 'code>(
        buffer: impl AsRef<[Instruction]>,
        offset: usize,
        ctx: &RuntimeFrame<'cp, 'code>,
    ) -> Option<(usize, Expression)>;
}

pub enum Expression {
    EmptyConstructor(EmptyConstructor),
    ReturnStatement(ReturnStatement),
    Super(EmptySuperCall),
    Comment(InstructionComment),
}

#[derive(Debug)]
pub struct InstructionComment(pub Instruction);

impl CheckExpression for InstructionComment {
    fn test<'cp, 'code>(
        instr: impl AsRef<[Instruction]>,
        offset: usize,
        _: &RuntimeFrame<'cp, 'code>,
    ) -> Option<(usize, Expression)> {
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

pub struct EmptyConstructor;

impl CheckExpression for EmptyConstructor {
    fn test<'cp, 'code>(
        buffer: impl AsRef<[Instruction]>,
        offset: usize,
        _: &RuntimeFrame<'cp, 'code>,
    ) -> Option<(usize, Expression)> {
        if buffer.as_ref().len() != 3 {
            return None;
        }

        let result = OpSeq([
            Op::Aload0,        // push this to stack
            Op::Invokespecial, // call this.<init>
            Op::Return,        // return with object on stack
        ])
        .test(buffer, 0);

        if !result {
            return None;
        }

        Some((3 - offset, Expression::EmptyConstructor(Self)))
    }
}

pub struct ReturnStatement; 

impl CheckExpression for ReturnStatement {
    fn test<'cp, 'code>(
        buffer: impl AsRef<[Instruction]>,
        offset: usize,
        _: &RuntimeFrame<'cp, 'code>,
    ) -> Option<(usize, Expression)> {
        let result = OpSeq([
            Op::Return, // return with object on stack
        ])
        .test(buffer, offset);

        if !result {
            return None;
        }

        Some((1, Expression::ReturnStatement(Self)))
    }
}

pub struct EmptySuperCall;

impl CheckExpression for EmptySuperCall {
    fn test<'cp, 'code>(
        buffer: impl AsRef<[Instruction]>,
        offset: usize,
        _: &RuntimeFrame<'cp, 'code>,
    ) -> Option<(usize, Expression)> {
        let result = OpSeq([
            Op::Aload0,        // push this to stack
            Op::Invokespecial, // call <init>
        ])
        .test(buffer, offset);

        if !result {
            return None;
        }

        Some((2, Expression::Super(Self)))
    }
}
