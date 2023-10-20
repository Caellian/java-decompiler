use crate::constant::ReferenceKind;
use crate::ty::JVMType;
use crate::ConstantTag;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConstantPoolError {
    #[error("accessing invalid constant pool index: {index}; length: {length}")]
    InvalidIndex { index: usize, length: usize },
    #[error("unexpected constant type: {found:?}; expected {expected:?}")]
    UnexpectedType {
        found: ConstantTag,
        expected: ConstantTag,
    },
}

#[derive(Error, Debug)]
pub enum ClassReadError {
    #[error("expected magic: 0xCAFEBABE; got 0x{found:X}")]
    InvalidMagic { found: u32 },

    #[error(transparent)]
    ConstantPoolIndex(#[from] ConstantPoolError),
    #[error("invalid reference to class constant")]
    InvalidClassReference,
    #[error("invalid reference to class name constant")]
    InvalidClassNameReference,
    #[error("invalid reference to interface constant or name constant")]
    InvalidInterfaceReference,

    #[error(transparent)]
    Constant(#[from] ConstantError),
    #[error(transparent)]
    ClassPath(#[from] ClassPathError),
    #[error(transparent)]
    AccessFlag(#[from] AccessFlagError),
    #[error(transparent)]
    Member(#[from] MemberError),
    #[error(transparent)]
    Attribute(#[from] AttributeError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum AccessFlagError {
    #[error("received one or more unsupported access flags 0x{found:X}")]
    InvalidValue { found: u16 },

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ConstantError {
    #[error("constant data abruptly ended")]
    IncompleteData,
    #[error("tag with ID {tag} isn't supported")]
    UnsupportedTag { tag: u8 },
    #[error("reference kind of 0x{value:X} isn't supported")]
    UnsupportedReferenceKind { value: u8 },
    #[error("invalid class path: {0}")]
    InvalidClassPath(String),

    #[error("utf8 error: {0}")]
    UTF8ParseError(#[from] std::string::FromUtf8Error),
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

impl From<num_enum::TryFromPrimitiveError<ConstantTag>> for ConstantError {
    fn from(primitive: num_enum::TryFromPrimitiveError<ConstantTag>) -> Self {
        ConstantError::UnsupportedTag {
            tag: primitive.number as u8,
        }
    }
}

impl From<num_enum::TryFromPrimitiveError<ReferenceKind>> for ConstantError {
    fn from(primitive: num_enum::TryFromPrimitiveError<ReferenceKind>) -> Self {
        ConstantError::UnsupportedReferenceKind {
            value: primitive.number as u8,
        }
    }
}

#[derive(Error, Debug)]
pub enum MemberError {
    #[error("class member name is pointing to a non-existent constant")]
    NoMemberName,
    #[error("class member name is pointing to a non-utf8 constant")]
    InvalidNameType,
    #[error("class member descriptor is pointing to a non-existent constant")]
    NoMemberDesc,

    #[error(transparent)]
    AccessFlagError(#[from] AccessFlagError),
    #[error(transparent)]
    AttributeReadError(#[from] AttributeError),
    #[error(transparent)]
    ConstantPool(#[from] ConstantPoolError),
    #[error(transparent)]
    JVMTypeError(#[from] JVMTypeError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum AttributeError {
    #[error("attribute name is pointing to a non-existent constant")]
    NoAttribName,
    #[error("attribute name is pointing to a non-utf8 constant")]
    InvalidNameType,
    #[error("attribute data wasn't fully read")]
    IncompleteData,
    #[error("attribute data is invalid")]
    InvalidData,
    #[error(transparent)]
    ConstantPool(#[from] ConstantPoolError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum ClassPathError {
    #[error("expected 'L' in byte stream for class path declaration; got '{0}'")]
    NoPrefix(char),
    #[error("expected ';' in byte stream for end of class path declaration")]
    NotTerminated,
    #[error("invalid identifier '{identifier}'; reason: {reason}")]
    InvalidIdentifier {
        identifier: String,
        reason: &'static str,
    },

    #[error("constant pool error: {0}")]
    ConstantPool(#[from] ConstantPoolError),

    #[error(transparent)]
    IOError(std::io::Error),
}

impl From<std::io::Error> for ClassPathError {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::UnexpectedEof => ClassPathError::NotTerminated,
            _ => ClassPathError::IOError(value),
        }
    }
}

#[derive(Error, Debug)]
pub enum JVMTypeError {
    #[error("invalid JVM type {found}; expected {expected}")]
    InvalidType { found: char, expected: &'static str },
    #[error("type is not a primitive")]
    NotPrimitive(JVMType),
    #[error("invalid type classpath; error: {0}")]
    ClassPath(#[from] ClassPathError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
