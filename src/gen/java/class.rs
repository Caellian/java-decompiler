use jvm_class_format::{AccessFlags, Class};
use std::io::{Cursor, Write};

use crate::gen::{
    java::{field::FieldContext, method::ClassContext},
    java::{JavaBackend, JavaContext, JavaScopeRequirements},
    indent::Indented,
    GenerateCode,
};

pub fn class_signature(access_flags: AccessFlags) -> String {
    let mut parts = Vec::with_capacity(4);

    // visibility is one of following
    if access_flags.contains(AccessFlags::PUBLIC) {
        parts.push("public");
    } else if access_flags.contains(AccessFlags::PROTECTED) {
        parts.push("protected");
    } else if access_flags.contains(AccessFlags::PRIVATE) {
        parts.push("private");
    }

    // inner classes can be static
    if access_flags.contains(AccessFlags::STATIC) {
        parts.push("static");
    }

    // a class can be abstract
    if access_flags.contains(AccessFlags::ABSTRACT) {
        parts.push("abstract");
    }

    // class inheritance can be prevented
    if access_flags.contains(AccessFlags::FINAL) {
        parts.push("final");
    }

    // class type
    if access_flags.contains(AccessFlags::ENUM) {
        parts.push("enum");
    } else if access_flags.contains(AccessFlags::INTERFACE) {
        parts.push("interface");
    } else if access_flags.contains(AccessFlags::ANNOTATION) {
        parts.push("@interface");
    } else {
        parts.push("class");
    }

    parts.join(" ")
}

impl GenerateCode<Class> for JavaBackend {
    fn write_value<W: std::io::Write>(
        &self,
        lang: &JavaContext,
        _: &(),
        class: &Class,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let mut req = JavaScopeRequirements::default();

        tracing::debug!("Generating class: {}", class.class_name);

        // TODO: don't clone constant_pool, pass it as a reference
        let lang = JavaContext {
            constant_pool: Some(class.constant_pool.clone()),
            ..(*lang).clone()
        };

        if lang.header_message.is_some() {
            let lines: Vec<&str> = lang.header_message.as_ref().unwrap().split('\n').collect();

            writeln!(w, "/*")?;
            for l in lines {
                writeln!(w, " * {}", l)?;
            }
            writeln!(w, " */")?;
        }

        if !class.class_name.inner_classes.is_empty() {
            todo!("handle inner classes")
        }

        let package_path = class.class_name.package_path();

        if !package_path.is_empty() {
            write!(w, "package {};\n\n", class.class_name.package_path())?;
        }

        let delayed = {
            let mut result = Vec::with_capacity(512);
            let mut w: Cursor<&mut Vec<u8>> = Cursor::new(&mut result);

            w.write_all(class_signature(class.access_flags).as_bytes())?;

            let class_name = class.class_name.clone();
            w.write_all(b" ")?;
            w.write_all(class_name.name.as_bytes())?;

            if class.super_name.is_some() && !class.super_name.as_ref().unwrap().is_object() {
                let super_name = class.super_name.as_ref().unwrap();
                w.write_all(b" extends ")?;
                w.write_all(super_name.name.as_bytes())?;
                req.imports.insert(super_name.clone());
            }

            if !class.interfaces.is_empty() {
                w.write_all(b" implements ")?;

                for (i, interface) in class.interfaces.iter().enumerate() {
                    req.imports.insert(interface.clone());
                    w.write_all(interface.name.as_bytes())?;

                    if i != class.interfaces.len() - 1 {
                        w.write_all(b", ")?;
                    }
                }
            }
            w.write_all(b" {\n")?;

            // TODO: generate enum entries

            tracing::debug!("- Generating fields for {}", class_name);

            let mut class_indent = Indented::new(
                &mut w,
                lang.indentation,
                1,
                b"{",
                b"}",
            );

            for field in &class.fields {
                let field_requirements =
                    self.write_value(&lang, &FieldContext, field, &mut class_indent)?;
                req.append(field_requirements.imports);
            }

            tracing::debug!("- Generating methods for {}", class_name);

            for method in &class.methods {
                let method_ctx = ClassContext {
                    class_name: class.class_name.clone(),
                    ..Default::default()
                };
                let method_requirements =
                    self.write_value(&lang, &method_ctx, method, &mut class_indent)?;
                req.append(method_requirements.imports);
            }

            w.write_all(b"}\n")?;
            w.flush()?;

            result
        };

        for import in req.imports.drain() {
            w.write_all(b"import ")?;
            w.write_all(import.full_path().as_bytes())?;
            w.write_all(b";\n")?;
        }
        w.write_all(b"\n")?;

        w.write_all(&delayed)?;

        tracing::debug!("- Done.");

        Ok(req)
    }
}
