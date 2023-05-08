use crate::error::ClassReadError;
use attribute::AttributeValue;
use byteorder::{ReadBytesExt, BE};
use error::ClassPathError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub use crate::access_flags::AccessFlags;
pub use crate::attribute::Attribute;
pub use crate::constant::{Constant, ConstantPool, ConstantTag};
pub use crate::member::Member;
pub use crate::op::{Instruction, Op};
pub use crate::ty::*;

pub mod access_flags;
pub mod attribute;
pub mod constant;
pub mod error;
pub mod ext;
pub mod member;
pub mod method;
pub mod op;
pub mod ty;

pub const CLASS_SIGNATURE: u32 = 0xCAFEBABE;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum SourceLanguage {
    Java,
    Kotlin,
    Scala,
    Groovy,
}

#[derive(Debug, Clone)]
pub struct CompilerInfo {
    pub major: u16,
    pub minor: u16,
    pub language: SourceLanguage,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassPath {
    pub package: Vec<String>,
    pub inner_classes: Vec<String>,
    pub name: String,
}

impl Default for ClassPath {
    fn default() -> Self {
        ClassPath::java_lang_class("Object")
    }
}

pub enum ClassPathParseState {
    Init { with_prefix: bool },
}

impl ClassPath {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Self, ClassPathError> {
        let mut path_tokens = vec![];

        let mut last = '\0';
        let mut token = String::with_capacity(8);
        while let Some(curr) = r.read_u8().ok().map(|it| it as char) {
            if !curr.is_alphanumeric() && curr != '/' {
                last = curr;
                break;
            }

            if curr != '/' {
                // curr.is_alphanumeric() is true, append char to token
                token.push(curr);
                // advance
                continue;
            }
            // else curr is '/'

            if token.len() == 0 {
                if path_tokens.is_empty() {
                    return Err(ClassPathError::InvalidIdentifier {
                        identifier: token,
                        reason: "class path starts with a separator ('/')",
                    });
                } else {
                    return Err(ClassPathError::InvalidIdentifier {
                        identifier: token,
                        reason: "class path contains two consecutive separators (\"//\")",
                    });
                }
            }

            if token.chars().next().unwrap().is_numeric() {
                return Err(ClassPathError::InvalidIdentifier {
                    identifier: token,
                    reason: "class path and name (identifiers) can't start with a digit",
                });
            }

            path_tokens.push(token);
            token = String::with_capacity(8);
        }
        // push last (unterminated, or '$'/';' terminated) token
        if !token.is_empty() {
            path_tokens.push(token);
        }

        let (package, name) = path_tokens.split_at(path_tokens.len() - 1);
        let name = name[0].clone();

        let mut inner_classes = Vec::with_capacity(2);
        if last == '$' {
            let mut inner = String::with_capacity(8);
            while let Some(curr) = r.read_u8().ok().map(|it| it as char) {
                match curr as char {
                    '$' => {
                        inner_classes.push(inner);
                        inner = String::with_capacity(8);
                    }
                    c if c.is_alphanumeric() => {
                        inner.push(c);
                    }
                    _ => {
                        break;
                    }
                }
            }
            if !inner.is_empty() {
                inner_classes.push(inner);
            }
        }

        return Ok(ClassPath {
            package: package.to_vec(),
            inner_classes,
            name,
        });
    }

    pub fn parse(name: impl AsRef<str>) -> Result<Self, ClassPathError> {
        let mut cursor = std::io::Cursor::new(name.as_ref());
        Self::read_from(&mut cursor)
    }

    pub fn package_path(&self) -> String {
        self.package.join(".")
    }

    pub fn jar_path(&self) -> String {
        let mut builder = self.package.join("/");
        builder += self.name.as_str();
        if !self.inner_classes.is_empty() {
            builder += "$";
            builder += self.inner_classes.join("$").as_str();
        }
        builder += ".class";

        builder
    }

    pub fn full_path(&self) -> String {
        let mut builder: String = self.package.join(".");
        if !builder.is_empty() {
            builder += ".";
        }
        builder += self.name.to_string().as_str();

        for inner_c in &self.inner_classes {
            builder += format!(".{}", inner_c).as_str();
        }

        builder
    }

    pub fn is_in_java_lang(&self) -> bool {
        if self.package.len() != 2 {
            return false;
        }

        self.package[0] == "java" && self.package[1] == "lang"
    }

    pub fn is_object(&self) -> bool {
        self.is_in_java_lang() && self.inner_classes.is_empty() && self.name == "Object"
    }

    pub(crate) fn java_lang_class(name: impl ToString) -> Self {
        ClassPath {
            package: vec!["java".to_string(), "lang".to_string()],
            inner_classes: vec![],
            name: name.to_string(),
        }
    }
}

impl Display for ClassPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_path().as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub compiler_info: CompilerInfo,

    pub access_flags: AccessFlags,

    pub constant_pool: ConstantPool,

    pub class_name: ClassPath,
    pub super_name: Option<ClassPath>,
    pub interfaces: Vec<ClassPath>,

    pub fields: Vec<Member>,
    pub methods: Vec<Member>,

    pub attributes: HashMap<String, AttributeValue>,
}

fn name_from_class_index(
    index: usize,
    constant_pool: &ConstantPool,
) -> Result<Option<ClassPath>, ClassReadError> {
    Ok(if index != 0 {
        match constant_pool.get(&(index as u16)) {
            Some(it) => Some(match it {
                Constant::Class { name_index } => match constant_pool.get(name_index) {
                    Some(Constant::Utf8 { value }) => ClassPath::parse(value)?,
                    _ => return Err(ClassReadError::InvalidClassReference),
                },
                _ => return Err(ClassReadError::InvalidClassReference),
            }),
            None => None,
        }
    } else {
        None
    })
}

impl Class {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Class, ClassReadError> {
        let magic_number = r.read_u32::<BE>()?;
        if magic_number != CLASS_SIGNATURE {
            return Err(ClassReadError::InvalidMagic {
                found: magic_number,
            });
        }

        let minor = r.read_u16::<BE>()?;
        let major = r.read_u16::<BE>()?;

        let const_pool_size = r.read_u16::<BE>()? as usize;
        let mut constant_pool = ConstantPool::with_capacity(const_pool_size);

        let mut pos = 1;
        while pos < const_pool_size {
            let const_info = Constant::read_from(r)?;
            let tag = const_info.tag();

            constant_pool.insert(pos as u16, const_info);

            pos += match tag {
                ConstantTag::Long => 2,
                ConstantTag::Double => 2,
                _ => 1,
            };
        }

        let access_flags = AccessFlags::read_from(r)?;

        let class_const_index = r.read_u16::<BE>()? as usize;
        let class_name = name_from_class_index(class_const_index, &constant_pool)?
            .ok_or(ClassReadError::NoClassName)?;

        let super_const_index = r.read_u16::<BE>()? as usize;
        let super_name = name_from_class_index(super_const_index, &constant_pool)?;

        let interface_count = r.read_u16::<BE>()? as usize;
        let mut interfaces = Vec::with_capacity(interface_count);

        for _ in 0..interface_count {
            let interface_index = r.read_u16::<BE>()? as usize;
            let interface_name = name_from_class_index(interface_index, &constant_pool)?
                .ok_or(ClassReadError::InvalidInterfaceReference)?;

            interfaces.push(interface_name);
        }

        let field_count = r.read_u16::<BE>()? as usize;
        let mut fields = Vec::with_capacity(interface_count);

        for _ in 0..field_count {
            fields.push(Member::read_from(r, &constant_pool)?);
        }

        let method_count = r.read_u16::<BE>()? as usize;
        let mut methods = Vec::with_capacity(interface_count);

        for _ in 0..method_count {
            methods.push(Member::read_from(r, &constant_pool)?);
        }

        let mut attributes = AttributeValue::read_all(r, Some(&constant_pool))?;

        // TODO: Detect source language

        Ok(Class {
            compiler_info: CompilerInfo {
                major,
                minor,
                language: SourceLanguage::Java,
            },

            access_flags,

            constant_pool,

            class_name,
            super_name,
            interfaces,

            fields,
            methods,

            attributes,
        })
    }
}

#[cfg(test)]
mod class_path_tests {
    use crate::ty::JVMPrimitive;

    use super::*;

    #[test]
    fn class_path_parse() {
        let valid = &[
            "Ljava/lang/String;",
            "Lcom/example/Simple;",
            "Lcom/example/Of$Nested$Class;",
        ];

        let invalid = &[
            "no/l/Prefix;",
            "Lno/semicolon/Suffix",
            "Lempty/child/Name$;",
        ];

        for v in valid {
            assert!(ClassPath::parse(v).is_ok(), "unable to parse");
        }

        for v in valid {
            assert!(ClassPath::parse(v).is_err(), "invalid sample parsed ok");
        }
    }
}
