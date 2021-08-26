use std::io::Read;

use byteorder::{ReadBytesExt, BE};

use crate::class::access_flags::AccessFlags;
use crate::class::attribute::Attribute;
use crate::class::constant::{Constant, ConstantPool, ConstantTag};
use crate::class::member::Member;
use crate::error::ClassReadError;
use std::fmt::{Display, Formatter};

pub mod access_flags;
pub mod attribute;
pub mod constant;
pub mod member;
pub mod method;
pub mod op;
pub mod r#type;

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
    major: u16,
    minor: u16,
    language: SourceLanguage,
}

#[derive(Debug, Clone)]
pub struct ClassPath {
    pub package: Vec<String>,
    pub outer_classes: Vec<String>,
    pub name: String,
}

impl ClassPath {
    pub fn from_string(name: &str) -> ClassPath {
        let mut package: Vec<String> = name.split('/').map(|s| s.to_string()).collect();

        let class = package.remove(package.len() - 1);
        let mut outer_classes: Vec<String> = class.split('$').map(|s| s.to_string()).collect();

        let name = outer_classes.remove(outer_classes.len() - 1);

        ClassPath {
            package,
            outer_classes,
            name,
        }
    }

    pub fn package_path(&self) -> String {
        self.package.join(".")
    }

    pub fn jar_path(&self) -> String {
        let mut builder = self.package.join("/");
        if !self.outer_classes.is_empty() {
            builder += self.outer_classes.join("$").as_str();
            builder += "$"
        }
        builder += self.name.as_str();
        builder += ".class";

        builder
    }

    pub fn full_path(&self) -> String {
        let mut builder: String = self.package.join(".");
        if !builder.is_empty() {
            builder += ".";
        }
        for outer_c in &self.outer_classes {
            builder += format!("{}.", outer_c).as_str();
        }
        builder += self.name.to_string().as_str();

        builder
    }

    pub fn is_in_java_lang(&self) -> bool {
        if self.package.len() != 2 {
            return false;
        }

        self.package[0] == "java" && self.package[1] == "lang"
    }

    pub fn is_object(&self) -> bool {
        self.is_in_java_lang() && self.outer_classes.is_empty() && self.name == "Object"
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

    pub attributes: Vec<Attribute>,
}

fn name_from_class_index(
    index: usize,
    constant_pool: &ConstantPool,
) -> Result<Option<ClassPath>, ClassReadError> {
    Ok(if index != 0 {
        match constant_pool.get(&index) {
            Some(it) => Some(match it {
                Constant::Class { name_index } => match &constant_pool[&(*name_index as usize)] {
                    Constant::Utf8 { value } => ClassPath::from_string(value),
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

            constant_pool.insert(pos, const_info);

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

        let attributes_count = r.read_u16::<BE>()? as usize;
        let mut attributes = Vec::with_capacity(interface_count);

        for _ in 0..attributes_count {
            attributes.push(Attribute::read_from(r, &constant_pool)?);
        }

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
