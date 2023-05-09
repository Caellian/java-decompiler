use jvm_class_format::attribute::CodeData;

use crate::{
    gen::{GenerateCode, GeneratorBackend},
    ir::{
        decompile,
        expression::{EmptySuperCall, Expression, InstructionComment},
        frame::RuntimeBase,
    },
};

use super::{JavaBackend, JavaContext, JavaScopeRequirements};

#[derive(Debug, Clone, Copy)]
pub struct MethodContext<'a> {
    pub is_constructor: bool,

    pub code: &'a CodeData,
}

impl<'a, C: AsRef<[u8]>> GenerateCode<C, MethodContext<'a>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &JavaContext,
        ctx: &MethodContext<'a>,
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

        let ir = decompile(&runtime_pool, *ctx, code);

        for expression in ir {
            self.write_value(lang, ctx, &expression, w)?;
        }

        Ok(req)
    }
}

impl GenerateCode<Expression, MethodContext<'_>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        _ctx: &MethodContext,
        input: &Expression,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        #[allow(unreachable_patterns)]
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
        w.write(b"// asm: ")?;
        w.write(input.0.op.name().as_bytes())?;
        for arg in &input.0.args {
            write!(w, " 0x{:X}", *arg)?;
        }
        w.write(b"\n")?;

        Ok(Default::default())
    }
}
