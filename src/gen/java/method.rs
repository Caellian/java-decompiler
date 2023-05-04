use jvm_class_format::{
    attribute::{AsData, CodeData, MethodParameterData},
    AccessFlags, ClassPath, Constant, Member,
};

use crate::gen::{
    java::TypeContext,
    java::{JavaBackend, JavaScopeRequirements},
    Generate,
};

use super::code::CodeContext;

#[derive(Debug, Default)]
pub struct MethodContext {
    pub class_name: ClassPath,

    pub synthetic: bool,
}

impl MethodContext {
    pub fn signature(access_flags: AccessFlags) -> String {
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
}

impl Generate<Member, MethodContext> for JavaBackend {
    fn write_value<W: std::io::Write>(
        lang: &Self::LanguageContext,
        ctx: &MethodContext,
        method: &Member,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let mut req = JavaScopeRequirements::default();

        if ctx.synthetic {
            w.write_all(b"// synthetic method\n")?;
        }

        w.write_all(MethodContext::signature(method.access_flags).as_bytes())?;

        if !method.is_constructor() {
            let (tn, method_req) =
                JavaBackend::to_string(lang, &TypeContext, &method.descriptor.value)?;
            req.append(method_req.imports);

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
                let (arg_type, tr) = JavaBackend::to_string(lang, &TypeContext, arg)?;
                req.append(tr.imports);

                let arg_name: String = if let Some(param) = params.and_then(|it| it.get(i)) {
                    let flags = param.access_flags;

                    if let Some(Constant::Utf8 { value }) = lang
                        .constant_pool
                        .as_ref()
                        .and_then(|it| it.get(&(param.name_index as usize)))
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

        let code: &CodeData = method
            .attributes
            .get("Code")
            .expect("expected a code attribute")
            .as_data()
            .unwrap();
        let code_ctx = CodeContext::from(code);
        let code_req = JavaBackend::write_value(lang, &code_ctx, &code.code, w)?;
        req.append(code_req.imports);

        writeln!(w, "}}\n")?;

        Ok(req)
    }
}
