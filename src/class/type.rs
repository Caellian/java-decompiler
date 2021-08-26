use crate::class::ClassPath;
use crate::error::JVMTypeError;
use byteorder::ReadBytesExt;
use std::io::{Cursor, Read, Seek, SeekFrom};

#[derive(Debug, Clone)]
pub enum JVMType {
    TByte,
    TChar,
    TDouble,
    TFloat,
    TInt,
    TLong,
    TShort,
    TBoolean,
    TVoid,
    TClass(ClassPath),
    TArray(Box<JVMType>),
}

impl JVMType {
    pub fn read_from<R: Read>(r: &mut R) -> Result<JVMType, JVMTypeError> {
        let c = r.read_u8()? as char;

        Ok(match c {
            'B' => JVMType::TByte,
            'C' => JVMType::TChar,
            'D' => JVMType::TDouble,
            'F' => JVMType::TFloat,
            'I' => JVMType::TInt,
            'J' => JVMType::TLong,
            'S' => JVMType::TShort,
            'Z' => JVMType::TBoolean,
            'V' => JVMType::TVoid,
            'L' => {
                let mut reference = String::with_capacity(8);
                let mut next = r.read_u8()? as char;
                while next != ';' {
                    reference.push(next);
                    next = r.read_u8()? as char;
                }

                JVMType::TClass(ClassPath::from_string(&reference))
            }
            '[' => JVMType::TArray(Box::new(JVMType::read_from(r)?)),
            _ => return Err(JVMTypeError::InvalidType { found: c }),
        })
    }

    pub fn from_str(value: &str) -> Result<JVMType, JVMTypeError> {
        let mut c = Cursor::new(value);
        JVMType::read_from(&mut c)
    }

    pub fn from_string(value: String) -> Result<JVMType, JVMTypeError> {
        let mut c = Cursor::new(value);
        JVMType::read_from(&mut c)
    }

    pub fn array_depth(&self) -> usize {
        match self {
            JVMType::TArray(inner) => 1 + inner.array_depth(),
            _ => 0,
        }
    }

    pub fn strip_arrays(&self) -> &JVMType {
        match self {
            JVMType::TArray(inner) => inner.strip_arrays(),
            _ => self,
        }
    }

    pub fn primitive(&self) -> Option<&'static str> {
        Some(match self {
            JVMType::TByte => "byte",
            JVMType::TChar => "char",
            JVMType::TDouble => "double",
            JVMType::TFloat => "float",
            JVMType::TInt => "int",
            JVMType::TLong => "long",
            JVMType::TShort => "short",
            JVMType::TBoolean => "boolean",
            JVMType::TVoid => "void",
            _ => return None,
        })
    }

    pub fn class(&self) -> Option<ClassPath> {
        Some(match self {
            JVMType::TByte => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Byte".to_string(),
            },
            JVMType::TChar => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Character".to_string(),
            },
            JVMType::TDouble => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Double".to_string(),
            },
            JVMType::TFloat => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Float".to_string(),
            },
            JVMType::TInt => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Integer".to_string(),
            },
            JVMType::TLong => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Long".to_string(),
            },
            JVMType::TShort => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Short".to_string(),
            },
            JVMType::TBoolean => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Boolean".to_string(),
            },
            JVMType::TVoid => ClassPath {
                package: vec!["java".to_string(), "lang".to_string()],
                outer_classes: vec![],
                name: "Void".to_string(),
            },
            JVMType::TClass(inner) => inner.clone(),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Descriptor {
    return_type: JVMType,
    argument_types: Vec<JVMType>,
}

impl Descriptor {
    pub fn read_from<R: Read + Seek>(r: &mut R) -> Result<Descriptor, JVMTypeError> {
        let mut next = r.read_u8()? as char;

        Ok(if next == '(' {
            let mut argument_types = Vec::new();

            while next != ')' {
                argument_types.push(JVMType::read_from(r)?);
                next = r.read_u8()? as char;
                r.seek(SeekFrom::Current(-1))?;
            }

            r.seek(SeekFrom::Current(1))?;

            Descriptor {
                return_type: JVMType::read_from(r)?,
                argument_types,
            }
        } else {
            r.seek(SeekFrom::Current(-1))?;

            Descriptor {
                return_type: JVMType::read_from(r)?,
                argument_types: Vec::new(),
            }
        })
    }

    pub fn from_str(value: &str) -> Result<Descriptor, JVMTypeError> {
        let mut c = Cursor::new(value);
        Descriptor::read_from(&mut c)
    }

    pub fn from_string(value: String) -> Result<Descriptor, JVMTypeError> {
        let mut c = Cursor::new(value);
        Descriptor::read_from(&mut c)
    }
}
