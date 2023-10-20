use jvm_class_format::{attribute::CodeData, ConstantPool, Member};

use crate::{
    gen::{GenerateCode, GeneratorBackend},
    ir::expression::{EmptySuperCall, Expression, InstructionComment},
};

use super::JavaBackend;

pub type CodeGenContext<'m, 'data> = (&'m Member, &'data CodeData);

impl<'m, 'data> GenerateCode<Expression, CodeGenContext<'m, 'data>> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        ctx: &CodeGenContext,
        input: &Expression,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        #[allow(unreachable_patterns)]
        match input {
            Expression::Comment(it) => self.write_value(lang, ctx, it, w),
            Expression::Super(it) => self.write_value(lang, ctx, it, w),
            _ => todo!("unimplemented expression"),
        }
    }
}

impl<'m, 'data, B: GeneratorBackend> GenerateCode<EmptySuperCall, CodeGenContext<'m, 'data>> for B {
    fn write_value<W: std::io::Write>(
        &self,
        _: &Self::LanguageContext,
        _: &CodeGenContext<'m, 'data>,
        _: &EmptySuperCall,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        #[cfg(debug_assertions)]
        w.write_all(b"super();\n")?;
        Ok(Default::default())
    }
}

impl<'m, 'data, B: GeneratorBackend> GenerateCode<InstructionComment, CodeGenContext<'m, 'data>> for B {
    fn write_value<W: std::io::Write>(
        &self,
        _: &Self::LanguageContext,
        _: &CodeGenContext<'m, 'data>,
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
