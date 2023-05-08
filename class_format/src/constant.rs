use crate::error::ConstantError;
use crate::ext::ReadByteVecExt;
use byteorder::{ReadBytesExt, BE};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Read;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
    Utf8 = 1,
    Data = 2,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Constant {
    Class {
        name_index: u16,
    },
    Fieldref {
        class_index: u16,
        name_and_type_info: u16,
    },
    Methodref {
        class_index: u16,
        name_and_type_info: u16,
    },
    InterfaceMethodref {
        class_index: u16,
        name_and_type_info: u16,
    },
    String {
        string_index: u16,
    },
    Integer {
        value: i32,
    },
    Float {
        value: OrderedFloat<f32>,
    },
    Long {
        value: i64,
    },
    Double {
        value: OrderedFloat<f64>,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8 {
        value: String,
    },
    /// Not yet part of JVM spec.
    /// Suggested as replacement for never implemented Unicode tag
    /// to allow storing binary blobs in class files.
    Data {
        content: Vec<u8>,
    },
    MethodHandle {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

// Future changes: https://bugs.openjdk.java.net/browse/JDK-8161256
impl Constant {
    pub fn tag(&self) -> ConstantTag {
        match self {
            Constant::Utf8 { .. } => ConstantTag::Utf8,
            Constant::Data { .. } => ConstantTag::Data,
            Constant::Integer { .. } => ConstantTag::Integer,
            Constant::Float { .. } => ConstantTag::Float,
            Constant::Long { .. } => ConstantTag::Long,
            Constant::Double { .. } => ConstantTag::Double,
            Constant::Class { .. } => ConstantTag::Class,
            Constant::String { .. } => ConstantTag::String,
            Constant::Fieldref { .. } => ConstantTag::Fieldref,
            Constant::Methodref { .. } => ConstantTag::Methodref,
            Constant::InterfaceMethodref { .. } => ConstantTag::InterfaceMethodref,
            Constant::NameAndType { .. } => ConstantTag::NameAndType,
            Constant::MethodHandle { .. } => ConstantTag::MethodHandle,
            Constant::MethodType { .. } => ConstantTag::MethodType,
            Constant::Dynamic { .. } => ConstantTag::Dynamic,
            Constant::InvokeDynamic { .. } => ConstantTag::InvokeDynamic,
            Constant::Module { .. } => ConstantTag::Module,
            Constant::Package { .. } => ConstantTag::Package,
        }
    }

    pub fn read_from<R: Read>(r: &mut R) -> Result<Constant, ConstantError> {
        let tag = ConstantTag::try_from(r.read_u8()?)?;

        Ok(match tag {
            ConstantTag::Utf8 => {
                let len = r.read_u16::<BE>()? as usize;
                let buff = r.read_byte_vec(len)?;

                Constant::Utf8 {
                    value: String::from_utf8(buff)?,
                }
            }
            ConstantTag::Data => {
                let len = r.read_u16::<BE>()? as usize;
                let content = r.read_byte_vec(len)?;

                Constant::Data { content }
            }
            ConstantTag::Integer => Constant::Integer {
                value: r.read_i32::<BE>()?,
            },
            ConstantTag::Float => Constant::Float {
                value: OrderedFloat::from(r.read_f32::<BE>()?),
            },
            ConstantTag::Long => Constant::Long {
                value: r.read_i64::<BE>()?,
            },
            ConstantTag::Double => Constant::Double {
                value: OrderedFloat::from(r.read_f64::<BE>()?),
            },
            ConstantTag::Class => Constant::Class {
                name_index: r.read_u16::<BE>()?,
            },
            ConstantTag::String => Constant::String {
                string_index: r.read_u16::<BE>()?,
            },
            ConstantTag::Fieldref => Constant::Fieldref {
                class_index: r.read_u16::<BE>()?,
                name_and_type_info: r.read_u16::<BE>()?,
            },
            ConstantTag::Methodref => Constant::Methodref {
                class_index: r.read_u16::<BE>()?,
                name_and_type_info: r.read_u16::<BE>()?,
            },
            ConstantTag::InterfaceMethodref => Constant::InterfaceMethodref {
                class_index: r.read_u16::<BE>()?,
                name_and_type_info: r.read_u16::<BE>()?,
            },
            ConstantTag::NameAndType => Constant::NameAndType {
                name_index: r.read_u16::<BE>()?,
                descriptor_index: r.read_u16::<BE>()?,
            },
            ConstantTag::MethodHandle => Constant::MethodHandle {
                reference_kind: ReferenceKind::try_from(r.read_u8()?)?,
                reference_index: r.read_u16::<BE>()?,
            },
            ConstantTag::MethodType => Constant::MethodType {
                descriptor_index: r.read_u16::<BE>()?,
            },
            ConstantTag::Dynamic => Constant::Dynamic {
                bootstrap_method_attr_index: r.read_u16::<BE>()?,
                name_and_type_index: r.read_u16::<BE>()?,
            },
            ConstantTag::InvokeDynamic => Constant::InvokeDynamic {
                bootstrap_method_attr_index: r.read_u16::<BE>()?,
                name_and_type_index: r.read_u16::<BE>()?,
            },
            ConstantTag::Module => Constant::Module {
                name_index: r.read_u16::<BE>()?,
            },
            ConstantTag::Package => Constant::Package {
                name_index: r.read_u16::<BE>()?,
            },
        })
    }
}

pub type ConstantPool = HashMap<u16, Constant>;
