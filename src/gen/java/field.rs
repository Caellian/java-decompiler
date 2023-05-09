use jvm_class_format::{AccessFlags, Member};

use crate::gen::{
    java::{JavaBackend, JavaScopeRequirements},
    GenerateCode,
};

pub struct FieldContext;

impl FieldContext {
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

impl GenerateCode<Member, FieldContext> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &Self::LanguageContext,
        _c: &FieldContext,
        field: &Member,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let mut req = JavaScopeRequirements::default();

        w.write_all(FieldContext::signature(field.access_flags).as_bytes())?;
        w.write_all(b" ")?;

        let (type_name, type_req) = self.generate(lang, &(), &field.descriptor.value)?;
        req.append(type_req.imports);
        w.write_all(type_name.as_bytes())?;
        w.write_all(b" ")?;

        w.write_all(field.name.as_bytes())?;
        w.write(format!(";\n").as_bytes())?;

        Ok(req)
    }
}
