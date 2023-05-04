use super::{Generate, GeneratorBackend};
use jvm_class_format::{ClassPath, ConstantPool, JVMPrimitive, JVMType};
use std::collections::HashSet;

pub mod class;
pub mod field;
pub mod method;
pub mod code;

pub fn primitive_name(primitive: JVMPrimitive) -> &'static str {
    match primitive {
        JVMPrimitive::TByte => "byte",
        JVMPrimitive::TChar => "char",
        JVMPrimitive::TDouble => "double",
        JVMPrimitive::TFloat => "float",
        JVMPrimitive::TInt => "int",
        JVMPrimitive::TLong => "long",
        JVMPrimitive::TShort => "short",
        JVMPrimitive::TBoolean => "boolean",
        JVMPrimitive::TVoid => "void",
    }
}

pub struct Type;

pub struct TypeContext;

impl Generate<JVMType, TypeContext> for JavaBackend {
    fn write_value<W: std::io::Write>(
        _lang: &Self::LanguageContext,
        _c: &TypeContext,
        input: &JVMType,
        w: &mut W,
    ) -> Result<Self::ScopeRequirements, std::io::Error> {
        let mut req = JavaScopeRequirements::default();
        match input {
            JVMType::TPrimitive(primitive) => {
                w.write_all(primitive_name(*primitive).as_bytes())?;
            }
            JVMType::TClass(class) => {
                if !class.is_in_java_lang() {
                    req.imports.insert(class.clone());
                }
                w.write_all(class.name.as_bytes())?;
            }
            JVMType::TPrimitiveArray { depth, inner } => {
                w.write_all(primitive_name(*inner).as_bytes())?;
                w.write_all("[]".repeat(*depth).as_bytes())?;
            }
            JVMType::TClassArray { depth, inner } => {
                if !inner.is_in_java_lang() {
                    req.append([inner.clone()]);
                }
                w.write_all(inner.name.as_bytes())?;
                w.write_all("[]".repeat(*depth).as_bytes())?;
            }
        };

        Ok(req)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[repr(u32)]
pub enum JavaVersion {
    Unsupported = 0,
    Java1,
    Java5,
    Java6,
    Java7,
    Java8,
    Java9,
    Java10,
    Java11,
    Java12,
    Java13,
    Java14,
    Java15,
    Java16,
    Java17,
    Java18,
    Java19,
}

impl Default for JavaVersion {
    fn default() -> Self {
        Self::Java18
    }
}

#[derive(Debug, Default)]
pub struct JavaGeneratorBuilder {
    result: JavaContext,
}

impl JavaGeneratorBuilder {
    pub fn new() -> JavaGeneratorBuilder {
        Self::default()
    }

    pub fn version(mut self, version: JavaVersion) -> Self {
        self.result.target_version = version;
        self
    }

    pub fn header(mut self, header: impl ToString) -> Self {
        self.result.header_message = Some(header.to_string());
        self
    }

    pub fn build(self) -> JavaContext {
        self.result
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Indentation {
    Tabs,
    Spaces(usize),
}

#[derive(Debug, Clone)]
pub struct JavaContext {
    pub target_version: JavaVersion,

    pub header_message: Option<String>,
    pub indentation: Indentation,

    pub constant_pool: Option<ConstantPool>,
}

#[derive(Debug, Default)]
pub struct JavaScopeRequirements {
    pub imports: HashSet<ClassPath>,
}

impl JavaScopeRequirements {
    pub fn append<'a>(&mut self, imports: impl IntoIterator<Item = ClassPath> + 'a) {
        let iter = imports.into_iter();
        for import in iter {
            self.imports.insert(import);
        }
    }
}

pub struct JavaBackend;
impl GeneratorBackend for JavaBackend {
    const NAME: &'static str = "Java";

    type LanguageContext = JavaContext;
    type ScopeRequirements = JavaScopeRequirements;
}

impl Default for JavaContext {
    fn default() -> JavaContext {
        JavaContext {
            target_version: JavaVersion::default(),
            header_message: Some(
                "Generated file - do not edit, your changes will be lost.".to_string(),
            ),
            indentation: Indentation::Spaces(4),
            constant_pool: None,
        }
    }
}
