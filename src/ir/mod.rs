pub mod expression;
pub mod frame;

use jvm_class_format::attribute::CodeData;
use jvm_class_format::{ConstantPool, Instruction, Member};

use crate::test_many_expr;

use expression::*;
use frame::*;

// JVM spec, pg. 620 - 15.12.4. Run-Time Evaluation of Method Invocation

pub fn decompile<'cp, 'code>(
    constant_pool: &'cp ConstantPool,
    method: &Member,
    code: &CodeData,
) -> Vec<Expression> {
    let instructions = Instruction::from_bytecode(&code.code);

    let frame = RuntimeFrame::new(constant_pool, code);

    let mut result = Vec::with_capacity(instructions.len());

    if method.is_constructor() {
        if let Some((_, expr)) = EmptyConstructor::test(&instructions, 0, &frame) {
            return vec![expr];
        }
    }

    let mut offset = 0;
    while offset < instructions.len() {
        #[rustfmt::skip]
        let (instruction_count, expr) = test_many_expr!(&[
            EmptySuperCall,
            InstructionComment
        ], &instructions, offset, &frame)
        .unwrap();

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
