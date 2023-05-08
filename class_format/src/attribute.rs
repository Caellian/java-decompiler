use byteorder::{ReadBytesExt, BE};
use paste::paste;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use crate::{error::AttributeError, ext::ReadByteVecExt, Constant, ConstantPool};

macro_rules! flat_entry {
    ($name:ident {$($entry:ident:$entry_t:ty,)+}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
            pub $entry: $entry_t
            ),+
        }

        impl $name {
            pub fn read_from<R: Read>(r: &mut R) -> Result<$name, std::io::Error> {
                Ok($name {
                    $(
                    $entry: paste! { r.[<read_ $entry_t>]::<BE>()? }
                    ),+
                })
            }
        }
    };
}

flat_entry!(ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
});

flat_entry!(LineNumber {
    start_pc: u16,
    line_number: u16,
});

flat_entry!(InnerClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
});

flat_entry!(LocalVariable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
});

flat_entry!(LocalVariableType {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
});

flat_entry!(MethodParameter {
    name_index: u16,
    access_flags: u16,
});

pub struct Annotation {
    pub type_index: u16,
}

pub trait Attribute: Into<AttributeValue> + Sized {
    const NAME: &'static str;

    fn read_data<R: std::io::Read>(
        r: &mut R,
        constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError>;
}

pub trait AsData<D: Attribute> {
    fn as_data(&self) -> Result<&D, AttributeError>;
}

#[derive(Debug, Clone)]
pub struct CodeData {
    pub max_stack: usize,
    pub max_locals: usize,
    pub code: Vec<u8>,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes: HashMap<String, AttributeValue>,
}

impl Attribute for CodeData {
    const NAME: &'static str = "Code";

    #[inline]
    fn read_data<R: std::io::Read>(
        r: &mut R,
        constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let max_stack = r.read_u16::<BE>()? as usize;
        let max_locals = r.read_u16::<BE>()? as usize;

        let code_length = r.read_u32::<BE>()? as usize;
        let code = r.read_byte_vec(code_length)?;

        let exception_table_length = r.read_u16::<BE>()? as usize;
        let mut exception_table = Vec::with_capacity(exception_table_length);

        for _ in 0..exception_table_length {
            exception_table.push(ExceptionTableEntry::read_from(r)?)
        }

        let attributes = AttributeValue::read_all(r, constant_pool)?;

        Ok(CodeData {
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        })
    }
}

impl AsData<CodeData> for AttributeValue {
    fn as_data(&self) -> Result<&CodeData, AttributeError> {
        match self {
            AttributeValue::Code(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<CodeData> for AttributeValue {
    fn from(value: CodeData) -> Self {
        AttributeValue::Code(value)
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionData {
    pub exceptions: Vec<usize>,
}

impl Attribute for ExceptionData {
    const NAME: &'static str = "Exceptions";

    #[inline]
    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let number_of_exceptions = r.read_u16::<BE>()? as usize;
        let mut exceptions = Vec::with_capacity(number_of_exceptions);

        for _ in 0..number_of_exceptions {
            exceptions.push(r.read_u16::<BE>()? as usize);
        }

        Ok(ExceptionData { exceptions })
    }
}

impl AsData<ExceptionData> for AttributeValue {
    fn as_data(&self) -> Result<&ExceptionData, AttributeError> {
        match self {
            AttributeValue::Exceptions(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<ExceptionData> for AttributeValue {
    fn from(value: ExceptionData) -> Self {
        AttributeValue::Exceptions(value)
    }
}

#[derive(Debug, Clone)]
pub struct InnerClassData {
    pub classes: Vec<InnerClass>,
}

impl Attribute for InnerClassData {
    const NAME: &'static str = "InnerClasses";

    #[inline]
    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let number_of_classes = r.read_u16::<BE>()? as usize;
        let mut classes = Vec::with_capacity(number_of_classes);

        for _ in 0..number_of_classes {
            classes.push(InnerClass::read_from(r)?);
        }

        Ok(InnerClassData { classes })
    }
}

impl AsData<InnerClassData> for AttributeValue {
    fn as_data(&self) -> Result<&InnerClassData, AttributeError> {
        match self {
            AttributeValue::InnerClasses(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<InnerClassData> for AttributeValue {
    fn from(value: InnerClassData) -> Self {
        AttributeValue::InnerClasses(value)
    }
}

#[derive(Debug, Clone)]
pub struct LineNumberTable {
    pub table: Vec<LineNumber>,
}

impl Attribute for LineNumberTable {
    const NAME: &'static str = "LineNumberTable";

    #[inline]
    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let line_number_table_length = r.read_u16::<BE>()? as usize;
        let mut table = Vec::with_capacity(line_number_table_length);

        for _ in 0..line_number_table_length {
            table.push(LineNumber::read_from(r)?);
        }

        Ok(LineNumberTable { table })
    }
}

impl AsData<LineNumberTable> for AttributeValue {
    fn as_data(&self) -> Result<&LineNumberTable, AttributeError> {
        match self {
            AttributeValue::LineNumberTable(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<LineNumberTable> for AttributeValue {
    fn from(value: LineNumberTable) -> Self {
        AttributeValue::LineNumberTable(value)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVariableTable {
    pub table: Vec<LocalVariable>,
}

impl Attribute for LocalVariableTable {
    const NAME: &'static str = "LocalVariableTable";

    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let local_variable_table_length = r.read_u16::<BE>()? as usize;
        let mut table = Vec::with_capacity(local_variable_table_length);

        for _ in 0..local_variable_table_length {
            table.push(LocalVariable::read_from(r)?);
        }

        Ok(LocalVariableTable { table })
    }
}

impl AsData<LocalVariableTable> for AttributeValue {
    fn as_data(&self) -> Result<&LocalVariableTable, AttributeError> {
        match self {
            AttributeValue::LocalVariableTable(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<LocalVariableTable> for AttributeValue {
    fn from(value: LocalVariableTable) -> Self {
        AttributeValue::LocalVariableTable(value)
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationDefaultData {
    pub default: Vec<u8>,
}

impl Attribute for AnnotationDefaultData {
    const NAME: &'static str = "AnnotationDefault";

    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let mut default = Vec::with_capacity(256);
        r.read_to_end(&mut default)?;
        Ok(AnnotationDefaultData { default })
    }
}

impl AsData<AnnotationDefaultData> for AttributeValue {
    fn as_data(&self) -> Result<&AnnotationDefaultData, AttributeError> {
        match self {
            AttributeValue::AnnotationDefault(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<AnnotationDefaultData> for AttributeValue {
    fn from(value: AnnotationDefaultData) -> Self {
        AttributeValue::AnnotationDefault(value)
    }
}

#[derive(Debug, Clone)]
pub struct EnclosingMethodData {
    pub class_index: usize,
    pub method_index: usize,
}

impl Attribute for EnclosingMethodData {
    const NAME: &'static str = "EnclosingMethod";

    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        Ok(EnclosingMethodData {
            class_index: r.read_u16::<BE>()? as usize,
            method_index: r.read_u16::<BE>()? as usize,
        })
    }
}

impl AsData<EnclosingMethodData> for AttributeValue {
    fn as_data(&self) -> Result<&EnclosingMethodData, AttributeError> {
        match self {
            AttributeValue::EnclosingMethod(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<EnclosingMethodData> for AttributeValue {
    fn from(value: EnclosingMethodData) -> Self {
        AttributeValue::EnclosingMethod(value)
    }
}

#[derive(Debug, Clone)]
pub struct LocalVariableTypeTable {
    pub table: Vec<LocalVariableType>,
}

impl Attribute for LocalVariableTypeTable {
    const NAME: &'static str = "LocalVariableTypeTable";

    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let local_variable_type_table_length = r.read_u16::<BE>()? as usize;
        let mut table = Vec::with_capacity(local_variable_type_table_length);

        for _ in 0..local_variable_type_table_length {
            table.push(LocalVariableType::read_from(r)?);
        }

        Ok(LocalVariableTypeTable { table })
    }
}

impl AsData<LocalVariableTypeTable> for AttributeValue {
    fn as_data(&self) -> Result<&LocalVariableTypeTable, AttributeError> {
        match self {
            AttributeValue::LocalVariableTypeTable(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<LocalVariableTypeTable> for AttributeValue {
    fn from(value: LocalVariableTypeTable) -> Self {
        AttributeValue::LocalVariableTypeTable(value)
    }
}

#[derive(Debug, Clone)]
pub struct SignatureData {
    pub signature: String,
}

impl Attribute for SignatureData {
    const NAME: &'static str = "Signature";
    fn read_data<R: std::io::Read>(
        r: &mut R,
        constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let index = r.read_u16::<BE>()?;
        match constant_pool.and_then(|cp| cp.get(&index)) {
            Some(Constant::Utf8 { value }) => Ok(SignatureData {
                signature: value.clone(),
            }),
            _ => return Err(AttributeError::MissingContant),
        }
    }
}

impl AsData<SignatureData> for AttributeValue {
    fn as_data(&self) -> Result<&SignatureData, AttributeError> {
        match self {
            AttributeValue::Signature(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<SignatureData> for AttributeValue {
    fn from(value: SignatureData) -> Self {
        AttributeValue::Signature(value)
    }
}

#[derive(Debug, Clone)]
pub struct MethodParameterData {
    pub parameters: Vec<MethodParameter>,
}

impl Attribute for MethodParameterData {
    const NAME: &'static str = "MethodParameters";

    fn read_data<R: std::io::Read>(
        r: &mut R,
        _constant_pool: Option<&ConstantPool>,
    ) -> Result<Self, AttributeError> {
        let length = r.read_u8()? as usize;
        let mut parameters = Vec::with_capacity(length);
        for _ in 0..length {
            let parameter = MethodParameter::read_from(r)?;
            parameters.push(parameter);
        }

        Ok(MethodParameterData { parameters })
    }
}

impl AsData<MethodParameterData> for AttributeValue {
    fn as_data(&self) -> Result<&MethodParameterData, AttributeError> {
        match self {
            AttributeValue::MethodParameters(it) => Ok(it),
            _ => Err(AttributeError::InvalidData),
        }
    }
}

impl From<MethodParameterData> for AttributeValue {
    fn from(value: MethodParameterData) -> Self {
        AttributeValue::MethodParameters(value)
    }
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Unknown { name: String, data: Vec<u8> },
    Code(CodeData),
    ConstantValue(u16),
    Deprecated,
    Exceptions(ExceptionData),
    InnerClasses(InnerClassData),
    LineNumberTable(LineNumberTable),
    LocalVariableTable(LocalVariableTable),
    SourceFile(u16),
    Synthetic,
    AnnotationDefault(AnnotationDefaultData),
    EnclosingMethod(EnclosingMethodData),
    LocalVariableTypeTable(LocalVariableTypeTable),
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    Signature(SignatureData),
    SourceDebugExtension,
    StackMapTable,
    BootstrapMethods,
    MethodParameters(MethodParameterData),
    RuntimeInvisibleTypeAnnotations,
    RuntimeVisibleTypeAnnotations,
    Module,
    ModuleMainClass,
    ModulePackages,
    NestHost,
    NestMembers,
}

impl AttributeValue {
    pub fn read_from<R: std::io::Read>(
        r: &mut R,
        constant_pool: Option<&ConstantPool>,
    ) -> Result<(String, AttributeValue), AttributeError> {
        let name_i = r.read_u16::<BE>()?;
        let name = match constant_pool.and_then(|cp| cp.get(&name_i)) {
            Some(c) => match c {
                Constant::Utf8 { value } => value.clone(),
                _ => return Err(AttributeError::InvalidNameType),
            },
            None => return Err(AttributeError::NoAttribName),
        };

        let length = r.read_u32::<BE>()? as usize;
        let data = r.read_byte_vec(length)?;

        let value = Self::from_name_and_data(&name, &data, constant_pool)?;

        Ok((name, value))
    }

    pub fn read_all<R: std::io::Read>(
        r: &mut R,
        constant_pool: Option<&ConstantPool>,
    ) -> Result<HashMap<String, AttributeValue>, AttributeError> {
        let attributes_count = r.read_u16::<BE>()? as usize;
        let mut attributes = HashMap::with_capacity(attributes_count);

        for _ in 0..attributes_count {
            let (name, attrib) = AttributeValue::read_from(r, constant_pool)?;
            attributes.insert(name, attrib);
        }

        Ok(attributes)
    }

    pub fn from_name_and_data(
        name: impl AsRef<str>,
        data: &[u8],
        constant_pool: Option<&ConstantPool>,
    ) -> Result<AttributeValue, AttributeError> {
        let mut r = Cursor::new(data);

        Ok(match name.as_ref() {
            "Code" => AttributeValue::Code(CodeData::read_data(&mut r, constant_pool)?),
            "ConstantValue" => AttributeValue::ConstantValue(r.read_u16::<BE>()?),
            "Deprecated" => AttributeValue::Deprecated,
            "Exceptions" => {
                AttributeValue::Exceptions(ExceptionData::read_data(&mut r, constant_pool)?)
            }
            "InnerClasses" => {
                AttributeValue::InnerClasses(InnerClassData::read_data(&mut r, constant_pool)?)
            }
            "Signature" => {
                AttributeValue::Signature(SignatureData::read_data(&mut r, constant_pool)?)
            }
            "LineNumberTable" => {
                AttributeValue::LineNumberTable(LineNumberTable::read_data(&mut r, constant_pool)?)
            }
            "LocalVariableTable" => AttributeValue::LocalVariableTable(
                LocalVariableTable::read_data(&mut r, constant_pool)?,
            ),
            "SourceFile" => AttributeValue::SourceFile(r.read_u16::<BE>()?),
            "Synthetic" => AttributeValue::Synthetic,
            "AnnotationDefault" => AttributeValue::AnnotationDefault(
                AnnotationDefaultData::read_data(&mut r, constant_pool)?,
            ),
            "EnclosingMethod" => AttributeValue::EnclosingMethod(EnclosingMethodData::read_data(
                &mut r,
                constant_pool,
            )?),
            "LocalVariableTypeTable" => AttributeValue::LocalVariableTypeTable(
                LocalVariableTypeTable::read_data(&mut r, constant_pool)?,
            ),
            "MethodParameters" => AttributeValue::MethodParameters(MethodParameterData::read_data(
                &mut r,
                constant_pool,
            )?),
            other => AttributeValue::Unknown {
                name: other.to_string(),
                data: data.to_vec(),
            },
        })
    }
}
