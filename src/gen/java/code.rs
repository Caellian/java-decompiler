use std::collections::HashMap;

use jvm_class_format::{
    attribute::{AttributeValue, CodeData, ExceptionTableEntry},
    Instruction,
};

use crate::gen::Generate;

use super::{JavaBackend, JavaContext, JavaScopeRequirements};

pub struct CodeContext<'a> {
    pub max_stack: usize,
    pub max_locals: usize,
    pub exception_table: &'a [ExceptionTableEntry],
    pub attributes: &'a HashMap<String, AttributeValue>,
}

impl<'a> From<&'a CodeData> for CodeContext<'a> {
    fn from(value: &'a CodeData) -> Self {
        CodeContext {
            max_stack: value.max_stack,
            max_locals: value.max_locals,
            exception_table: &value.exception_table,
            attributes: &value.attributes,
        }
    }
}

impl<'a, C: AsRef<[u8]>> Generate<C, CodeContext<'a>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        _lang: &JavaContext,
        _c: &CodeContext<'a>,
        code: &C,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let req = JavaScopeRequirements::default();

        let instructions = Instruction::from_bytecode(code);

        for i in &instructions {
            match i.op {
                _ => {
                    w.write_all(b"/* ")?;
                    w.write_all(i.op.name().as_bytes())?;
                    for arg in &i.args {
                        w.write_all(b" ")?;
                        write!(w, "{:x}", *arg)?;
                    }
                    w.write_all(b" */\n")?;
                }
            }
        }

        Ok(req)
    }
}
