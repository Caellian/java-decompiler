use crate::class::access_flags::AccessFlags;
use crate::class::attribute::AttributeValue;
use crate::class::op::Op;
use crate::class::Class;
use crate::gen::CodeGenerator;
use std::convert::TryFrom;
use std::io::Write;

pub enum JavaVersion {
    Java17,
    Java16,
    Java15,
    Java14,
    Java13,
    Java12,
    Java11,
    Java10,
    Java9,
    Java8,
    Java7,
    Java6,
    Java5,
    Java1,
}

pub struct JavaGenerator {
    pub target_version: JavaVersion,
    pub header_message: Option<String>,
}

impl CodeGenerator for JavaGenerator {
    const NAME: &'static str = "Java";

    fn generate<W: Write>(&self, class: &Class, w: &mut W) -> Result<(), std::io::Error> {
        if self.header_message.is_some() {
            let lines: Vec<&str> = self.header_message.as_ref().unwrap().split('\n').collect();

            writeln!(w, "\\*")?;
            for l in lines {
                writeln!(w, " * {}", l)?;
            }
            writeln!(w, " *\\")?;
        }

        let package_path = class.class_name.package_path();

        if package_path.len() > 0 {
            write!(w, "package {};\n\n", class.class_name.package_path())?;
        }

        writeln!(w, "// imports")?; // TODO: Handle imports

        if class.access_flags.contains(AccessFlags::PUBLIC) {
            write!(w, "public")?;
        } else if class.access_flags.contains(AccessFlags::PROTECTED) {
            write!(w, "protected")?;
        } else if class.access_flags.contains(AccessFlags::PRIVATE) {
            write!(w, "private")?;
        }

        if class.access_flags.contains(AccessFlags::FINAL) {
            write!(w, " final")?;
        }

        if class.access_flags.contains(AccessFlags::ENUM) {
            write!(w, " enum")?;
        } else if class.access_flags.contains(AccessFlags::INTERFACE) {
            write!(w, " interface")?;
        } else if class.access_flags.contains(AccessFlags::ANNOTATION) {
            write!(w, " @interface")?;
        } else {
            write!(w, " class")?;
        }

        let class_name = class.class_name.clone();
        write!(w, " {}", class_name.name)?;

        if class.super_name.is_some() && !class.super_name.as_ref().unwrap().is_object() {
            write!(w, " extends {}", class.super_name.as_ref().unwrap().name)?;
        }

        if !class.interfaces.is_empty() {
            let last_i = class.interfaces.len() - 1;
            write!(w, " implements ")?;
            for (i, interface) in class.interfaces.iter().enumerate() {
                write!(w, "{}", interface.full_path())?; // TODO: Use resolved name after imports are done

                if i != last_i {
                    write!(w, ", ")?;
                }
            }
        }

        writeln!(w, " {{")?;

        // TODO: generate enum entries

        tracing::trace!("Generating fields for {}", class_name);

        tracing::trace!("Generating methods for {}", class_name);
        for m in &class.methods {
            if m.is_constructor() {
                continue; // TODO: Write
            }

            if m.access_flags.contains(AccessFlags::PUBLIC) {
                write!(w, "public")?;
            } else if m.access_flags.contains(AccessFlags::PROTECTED) {
                write!(w, "protected")?;
            } else if m.access_flags.contains(AccessFlags::PRIVATE) {
                write!(w, "private")?;
            }

            if m.access_flags.contains(AccessFlags::STATIC) {
                write!(w, " static")?;
            }

            write!(w, " {}", m.descriptor)?;

            write!(w, " {}(", m.name)?;
            writeln!(w, ") {{")?;

            let code = &m
                .attributes
                .iter()
                .find(|a| a.name == "Code")
                .unwrap()
                .value;
            match code {
                AttributeValue::Code { code, .. } => {
                    let mut pos = 0;
                    while pos < code.len() {
                        let instruction = Op::try_from(code[pos]).unwrap();

                        write!(w, "    {}", instruction.name())?;
                        let argc = instruction.argc();

                        for offset in 0..argc {
                            write!(w, " {}", code[pos + offset])?;
                        }
                        writeln!(w, "")?;

                        pos += 1 + argc;
                    }
                }
                _ => todo!("Return an error"),
            }

            writeln!(w, "}}\n")?;
        }

        writeln!(w, "}}")?;

        return Ok(());
    }
}
