use jvm_class_format::{Instruction, Op};


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
    (&[$first: ty $(,$other: ty) *], $instructions: expr, $offset: expr) => {
        <$first>::test($instructions, $offset)$(
            .or_else(|| <$other>::test($instructions, $offset)))*
    };
}

pub trait CheckExpression {
    fn test(buffer: impl AsRef<[Instruction]>, offset: usize) -> Option<(usize, Expression)>;
}

pub enum Expression {
    Super(EmptySuperCall),
    Comment(InstructionComment),
}

#[derive(Debug)]
pub struct InstructionComment(pub Instruction);

impl CheckExpression for InstructionComment {
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

impl CheckExpression for EmptySuperCall {
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
