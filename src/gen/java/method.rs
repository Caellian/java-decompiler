use std::io::Cursor;

use jvm_class_format::{
    attribute::{AsData, CodeData, MethodParameterData},
    AccessFlags, ClassPath, Constant, Member,
};

use crate::{
    gen::{
        java::{JavaBackend, JavaScopeRequirements},
        GenerateCode, GeneratorBackend, GeneratorVerbosity,
    },
    ir::decompile,
};

#[derive(Debug, Default)]
pub struct ClassContext {
    pub class_name: ClassPath,

    pub synthetic: bool,
}

pub fn method_signature(access_flags: AccessFlags) -> String {
    let mut result = String::with_capacity(64);
    if access_flags.contains(AccessFlags::PUBLIC) {
        result.push_str("public");
    } else if access_flags.contains(AccessFlags::PROTECTED) {
        result.push_str("protected");
    } else if access_flags.contains(AccessFlags::PRIVATE) {
        result.push_str("private");
    }

    if access_flags.contains(AccessFlags::STATIC) {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str("static");
    }

    result
}

impl GenerateCode<Member, ClassContext> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        ctx: &ClassContext,
        method: &Member,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let mut req = JavaScopeRequirements::default();

        let code: &CodeData = method
            .attributes
            .get("Code")
            .expect("expected a code attribute")
            .as_data()
            .unwrap();

        let constant_pool = lang.constant_pool.as_ref().expect("no contant pool");

        let expressions = decompile(constant_pool, method, code);

        let mut generated = Vec::new();
        {
            let mut gen_w = Cursor::new(&mut generated);
            for expression in expressions {
                let e_req = self.write_value(lang, &(method, code), &expression, &mut gen_w)?;
                req.include(e_req);
            }
        }

        if self.verbosity() == GeneratorVerbosity::All {
            if method.is_constructor()
                && method.descriptor.arguments.len() == 0
                && generated.len() == 0
            {
                return Ok(req);
            }
        }

        if ctx.synthetic {
            w.write_all(b"// synthetic method\n\n")?;
        }

        w.write_all(method_signature(method.access_flags).as_bytes())?;

        if !method.is_constructor() {
            let (tn, method_req) = self.generate(lang, &(), &method.descriptor.value)?;
            req.add_import(method_req.imports);

            write!(w, " {} {}(", tn, method.name)?;
        } else {
            write!(w, " {}(", ctx.class_name.name)?;
        }

        if !method.descriptor.arguments.is_empty() {
            let params = if let Some(MethodParameterData { parameters }) = method
                .attributes
                .get("MethodParameters")
                .and_then(|attr| attr.as_data().ok())
            {
                Some(parameters)
            } else {
                None
            };

            for (i, arg) in method.descriptor.arguments.iter().enumerate() {
                let (arg_type, tr) = self.generate(lang, &(), arg)?;
                req.add_import(tr.imports);

                let arg_name: String = if let Some(param) = params.and_then(|it| it.get(i)) {
                    // let flags = param.access_flags; // TODO: Check spec

                    if let Some(Constant::Utf8 { value }) = lang
                        .constant_pool
                        .as_ref()
                        .and_then(|it| it.try_get(param.name_index as usize).ok())
                    {
                        value.to_string()
                    } else {
                        format!("arg_{}", i)
                    }
                } else {
                    format!("arg_{}", i)
                };

                write!(w, "{} {}", arg_type, arg_name)?;
                if i < method.descriptor.arguments.len() - 1 {
                    write!(w, ", ")?;
                }
            }
        }
        w.write_all(b") {\n")?;
        w.write_all(&generated)?;
        w.write_all(b"}\n")?;

        Ok(req)
    }
}
