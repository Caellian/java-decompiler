use crate::class::constant::{ConstantTag, ReferenceKind};
use num_enum::TryFromPrimitiveError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClassReadError {
    #[error("expected magic: 0xCAFEBABE; got 0x{found:X}")]
    InvalidMagic { found: u32 },

    #[error("class name not found")]
    NoClassName,
    #[error("invalid reference to class constant or name constant")]
    InvalidClassReference,
    #[error("invalid reference to interface constant or name constant")]
    InvalidInterfaceReference,

    #[error(transparent)]
    ConstantReadError {
        #[from]
        inner: ConstantReadError,
    },
    #[error(transparent)]
    AccessFlagError {
        #[from]
        inner: AccessFlagError,
    },
    #[error(transparent)]
    MemberReadError {
        #[from]
        inner: MemberReadError,
    },
    #[error(transparent)]
    AttributeReadError {
        #[from]
        inner: AttributeReadError,
    },
    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum ConstantReadError {
    #[error("constant data abruptly ended")]
    IncompleteData,
    #[error("tag with ID {tag} isn't supported")]
    UnsupportedTag { tag: u8 },
    #[error("reference kind of 0x{value:X} isn't supported")]
    UnsupportedReferenceKind { value: u8 },

    #[error(transparent)]
    UTF8ParseError {
        #[from]
        inner: std::string::FromUtf8Error,
    },
    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

impl From<TryFromPrimitiveError<ConstantTag>> for ConstantReadError {
    fn from(primitive: TryFromPrimitiveError<ConstantTag>) -> Self {
        ConstantReadError::UnsupportedTag {
            tag: primitive.number as u8,
        }
    }
}

impl From<TryFromPrimitiveError<ReferenceKind>> for ConstantReadError {
    fn from(primitive: TryFromPrimitiveError<ReferenceKind>) -> Self {
        ConstantReadError::UnsupportedReferenceKind {
            value: primitive.number as u8,
        }
    }
}

#[derive(Error, Debug)]
pub enum MemberReadError {
    #[error("class member name is pointing to a non-existent constant")]
    NoMemberName,
    #[error("class member name is pointing to a non-utf8 constant")]
    InvalidNameType,
    #[error("class member descriptor is pointing to a non-existent constant")]
    NoMemberDesc,
    #[error("class member descriptor is pointing to a non-utf8 constant")]
    InvalidDescType,

    #[error(transparent)]
    AccessFlagError {
        #[from]
        inner: AccessFlagError,
    },
    #[error(transparent)]
    AttributeReadError {
        #[from]
        inner: AttributeReadError,
    },
    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum AccessFlagError {
    #[error("received one or more unsupported access flags 0x{found:X}")]
    InvalidValue { found: u16 },

    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum AttributeReadError {
    #[error("attribute name is pointing to a non-existent constant")]
    NoAttribName,
    #[error("attribute name is pointing to a non-utf8 constant")]
    InvalidNameType,
    #[error("attribute data wasn't fully read")]
    IncompleteData,
    #[error("attribute data is invalid")]
    InvalidData,

    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum JVMTypeError {
    #[error("invalid JVM type {found}")]
    InvalidType { found: char },

    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum ManifestParseError {
    #[error("misplaced continuation line")]
    MisplacedContinuation,
    #[error("invalid header field")]
    InvalidHeader,
    #[error("invalid manifest entry")]
    InvalidEntry,

    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}
