use crate::error::JVMTypeError;
use crate::ClassPath;
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JVMPrimitive {
    TByte,
    TChar,
    TDouble,
    TFloat,
    TInt,
    TLong,
    TShort,
    TBoolean,
    TVoid,
}

impl JVMPrimitive {
    pub fn class_path(self) -> ClassPath {
        match self {
            JVMPrimitive::TByte => ClassPath::java_lang_class("Byte"),
            JVMPrimitive::TChar => ClassPath::java_lang_class("Character"),
            JVMPrimitive::TDouble => ClassPath::java_lang_class("Double"),
            JVMPrimitive::TFloat => ClassPath::java_lang_class("Float"),
            JVMPrimitive::TInt => ClassPath::java_lang_class("Integer"),
            JVMPrimitive::TLong => ClassPath::java_lang_class("Long"),
            JVMPrimitive::TShort => ClassPath::java_lang_class("Short"),
            JVMPrimitive::TBoolean => ClassPath::java_lang_class("Boolean"),
            JVMPrimitive::TVoid => ClassPath::java_lang_class("Void"),
        }
    }
}

impl TryFrom<char> for JVMPrimitive {
    type Error = JVMTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'B' => JVMPrimitive::TByte,
            'C' => JVMPrimitive::TChar,
            'D' => JVMPrimitive::TDouble,
            'F' => JVMPrimitive::TFloat,
            'I' => JVMPrimitive::TInt,
            'J' => JVMPrimitive::TLong,
            'S' => JVMPrimitive::TShort,
            'Z' => JVMPrimitive::TBoolean,
            'V' => JVMPrimitive::TVoid,
            _ => {
                return Err(JVMTypeError::InvalidType {
                    found: value,
                    expected: "a primitive type",
                })
            }
        })
    }
}

impl TryFrom<JVMType> for JVMPrimitive {
    type Error = JVMTypeError;

    fn try_from(value: JVMType) -> Result<Self, Self::Error> {
        match value {
            JVMType::TPrimitive(it) => Ok(it),
            _ => Err(JVMTypeError::NotPrimitive(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JVMType {
    TPrimitive(JVMPrimitive),
    TClass(ClassPath),
    TPrimitiveArray { depth: usize, inner: JVMPrimitive },
    TClassArray { depth: usize, inner: ClassPath },
}

impl From<JVMPrimitive> for JVMType {
    fn from(prim: JVMPrimitive) -> Self {
        JVMType::TPrimitive(prim)
    }
}

impl From<ClassPath> for JVMType {
    fn from(cp: ClassPath) -> Self {
        JVMType::TClass(cp)
    }
}

#[derive(Debug, Clone)]
pub struct TypeSpecifier(pub JVMType);

impl JVMType {
    pub fn read_from<R: Read>(r: &mut R) -> Result<JVMType, JVMTypeError> {
        JVMType::read_from_with_prefix(r, None)
    }

    pub fn read_from_with_prefix<R: Read>(
        r: &mut R,
        prefix: Option<char>,
    ) -> Result<JVMType, JVMTypeError> {
        let c = match prefix {
            Some(p) => p,
            None => r.read_u8()? as char,
        };

        if let Ok(it) = JVMPrimitive::try_from(c) {
            return Ok(JVMType::TPrimitive(it));
        }

        Ok(match c {
            'L' => {
                let mut reference = String::with_capacity(8);
                let mut next = r.read_u8()? as char;
                while next != ';' {
                    reference.push(next);
                    next = r.read_u8()? as char;
                }

                JVMType::TClass(ClassPath::parse(&reference)?)
            }
            '[' => {
                let mut depth = 1;

                let mut next = r.read_u8()? as char;
                while next == '[' {
                    next = r.read_u8()? as char;
                    depth += 1;
                }

                if next == 'L' {
                    JVMType::TClassArray {
                        depth,
                        inner: ClassPath::read_from(r)?,
                    }
                } else {
                    JVMType::TPrimitiveArray {
                        depth,
                        inner: JVMPrimitive::try_from(next)?,
                    }
                }
            }
            _ => {
                return Err(JVMTypeError::InvalidType {
                    found: c,
                    expected: "a JVM type",
                })
            }
        })
    }

    pub fn from_string(value: String) -> Result<JVMType, JVMTypeError> {
        let mut c = Cursor::new(value);
        JVMType::read_from(&mut c)
    }

    pub fn array_depth(&self) -> usize {
        match self {
            JVMType::TClassArray { depth, .. } | JVMType::TPrimitiveArray { depth, .. } => *depth,
            _ => 0,
        }
    }

    pub fn strip_arrays(&self) -> JVMType {
        match self {
            JVMType::TClassArray { inner, .. } => JVMType::TClass(inner.clone()),
            JVMType::TPrimitiveArray { inner, .. } => JVMType::TPrimitive(*inner),
            _ => self.clone(),
        }
    }

    pub fn class(&self) -> Option<ClassPath> {
        Some(match self {
            JVMType::TPrimitive(primitive) => primitive.class_path(),
            JVMType::TClass(inner) => inner.clone(),
            _ => return None,
        })
    }
}

impl FromStr for JVMType {
    type Err = JVMTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cursor::new(s);
        JVMType::read_from(&mut c)
    }
}

#[derive(Debug, Clone)]
pub struct Descriptor {
    pub value: JVMType,
    pub arguments: Vec<JVMType>,
}

impl Descriptor {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Descriptor, JVMTypeError> {
        let mut next = r.read_u8()? as char;

        Ok(if next == '(' {
            let mut arguments = Vec::new();

            loop {
                next = r.read_u8()? as char;
                if next == ')' {
                    break;
                }
                let jvmt = JVMType::read_from_with_prefix(r, Some(next))?;
                arguments.push(jvmt);
            }

            Descriptor {
                value: JVMType::read_from(r)?,
                arguments,
            }
        } else {
            Descriptor {
                value: JVMType::read_from_with_prefix(r, Some(next))?,
                arguments: Vec::new(),
            }
        })
    }

    pub fn from_string(value: String) -> Result<Descriptor, JVMTypeError> {
        let mut c = Cursor::new(value);
        Descriptor::read_from(&mut c)
    }
}

impl FromStr for Descriptor {
    type Err = JVMTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = Cursor::new(s);
        Descriptor::read_from(&mut c)
    }
}

#[cfg(test)]
mod descriptor_tests {
    use super::*;

    #[test]
    fn descriptor_parsing_works() {
        let descriptors = &["(ILjava/lang/String;[I)J"];
    }
}
