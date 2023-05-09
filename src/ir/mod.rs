pub mod expression;
pub mod frame;

use jvm_class_format::Instruction;

use crate::gen::java::code::MethodContext;
use crate::test_many_expr;

use expression::*;
use frame::*;

// JVM spec, pg. 620 - 15.12.4. Run-Time Evaluation of Method Invocation

pub fn decompile<'a, 'r, 'c>(
    lang: &'r RuntimeBase,
    ctx: MethodContext<'c>,
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
        ], &instructions, offset)
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
