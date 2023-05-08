use std::collections::HashMap;

use jvm_class_format::attribute::{AttributeValue, CodeData, ExceptionTableEntry};

use crate::{
    gen::{GenerateCode, GeneratorBackend},
    ir::{to_ir, Expression, RuntimeBase, EmptySuperCall, InstructionComment},
};

use super::{JavaBackend, JavaContext, JavaScopeRequirements};

#[derive(Debug, Clone, Copy)]
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

impl<'a, C: AsRef<[u8]>> GenerateCode<C, CodeContext<'a>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &JavaContext,
        ctx: &CodeContext<'a>,
        code: &C,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let req = JavaScopeRequirements::default();

        let runtime_pool: RuntimeBase = RuntimeBase {
            constant_pool: lang
                .constant_pool
                .as_ref()
                .expect("no contant pool")
                .clone(),
        };

        let ir = to_ir(&runtime_pool, *ctx, code);

        for expression in ir {
            self.write_value(lang, ctx, &expression, w)?;
        }

        Ok(req)
    }
}

impl GenerateCode<Expression, CodeContext<'_>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        ctx: &CodeContext,
        input: &Expression,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        match input {
            Expression::Comment(ic) => self.write_value(lang, &(), ic, w),
            Expression::Super(it) => self.write_value(lang, &(), it, w),
            _ => todo!("unimplemented expression"),
        }
    }
}

impl<B: GeneratorBackend> GenerateCode<EmptySuperCall> for B {
    fn write_value<W: std::io::Write>(
        &self,
        _: &Self::LanguageContext,
        _: &(),
        _: &EmptySuperCall,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        #[cfg(debug_assertions)]
        w.write_all(b"super();\n")?;
        Ok(Default::default())
    }
}

impl<B: GeneratorBackend> GenerateCode<InstructionComment> for B {
    fn write_value<W: std::io::Write>(
        &self,
        _: &Self::LanguageContext,
        _: &(),
        input: &InstructionComment,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        w.write(b"// ")?;
        w.write(input.0.op.name().as_bytes())?;
        for arg in &input.0.args {
            write!(w, " 0x{:X}", *arg)?;
        }
        w.write(b"\n")?;

        Ok(Default::default())
    }
}
