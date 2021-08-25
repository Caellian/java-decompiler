use crate::class::constant::{Constant, ConstantPool};
use crate::error::AttributeReadError;
use byteorder::{ReadBytesExt, BE};
use paste::paste;
use std::io::{Cursor, Read};
use crate::ext::ReadByteVecExt;

macro_rules! flat_entry {
    ($name:ident {$($entry:ident:$entry_t:ty,)+}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
            $entry: $entry_t
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

pub struct Annotation {
    type_index: u16,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    Unknown(Vec<u8>),
    Code {
        max_stack: usize,
        max_locals: usize,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<Attribute>,
    },
    ConstantValue {
        value: usize,
    },
    Deprecated,
    Exceptions {
        exceptions: Vec<usize>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    LineNumberTable {
        table: Vec<LineNumber>,
    },
    LocalVariableTable {
        table: Vec<LocalVariable>,
    },
    SourceFile {
        value: usize,
    },
    Synthetic,
    AnnotationDefault {
        default: Vec<u8>,
    },
    EnclosingMethod {
        class_index: usize,
        method_index: usize,
    },
    LocalVariableTypeTable {
        table: Vec<LocalVariableType>,
    },
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    Signature {
        signature: String, // TODO: Specialize - write a parser
    },
    SourceDebugExtension,
    StackMapTable,
    BootstrapMethods,
    MethodParameters,
    RuntimeInvisibleTypeAnnotations,
    RuntimeVisibleTypeAnnotations,
    Module,
    ModuleMainClass,
    ModulePackages,
    NestHost,
    NestMembers,
}

impl AttributeValue {
    pub fn from_name(
        name: &str,
        data: &[u8],
        constant_pool: &ConstantPool,
    ) -> Result<AttributeValue, AttributeReadError> {
        let mut c = Cursor::new(data);

        Ok(match name {
            "Code" => {
                let max_stack = c.read_u16::<BE>()? as usize;
                let max_locals = c.read_u16::<BE>()? as usize;

                let code_length = c.read_u32::<BE>()? as usize;
                let code = c.read_byte_vec(code_length)?;

                let exception_table_length = c.read_u16::<BE>()? as usize;
                let mut exception_table = Vec::with_capacity(exception_table_length);

                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionTableEntry::read_from(&mut c)?)
                }

                let attributes_count = c.read_u16::<BE>()? as usize;
                let mut attributes = Vec::with_capacity(attributes_count);

                for _ in 0..attributes_count {
                    attributes.push(Attribute::read_from(&mut c, &constant_pool)?);
                }

                AttributeValue::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                }
            }
            "ConstantValue" => AttributeValue::ConstantValue {
                value: c.read_u16::<BE>()? as usize,
            },
            "Deprecated" => AttributeValue::Deprecated,
            "Exceptions" => {
                let number_of_exceptions = c.read_u16::<BE>()? as usize;
                let mut exceptions = Vec::with_capacity(number_of_exceptions);

                for _ in 0..number_of_exceptions {
                    exceptions.push(c.read_u16::<BE>()? as usize);
                }

                AttributeValue::Exceptions { exceptions }
            }
            "InnerClasses" => {
                let number_of_classes = c.read_u16::<BE>()? as usize;
                let mut classes = Vec::with_capacity(number_of_classes);

                for _ in 0..number_of_classes {
                    classes.push(InnerClass::read_from(&mut c)?);
                }

                AttributeValue::InnerClasses { classes }
            }
            "Signature" => {
                match constant_pool
                    .get(&(c.read_u16::<BE>()? as usize))
                    .ok_or(AttributeReadError::InvalidData)?
                {
                    Constant::Utf8 { value } => AttributeValue::Signature {
                        signature: value.clone(),
                    },
                    _ => return Err(AttributeReadError::InvalidData),
                }
            }
            "LineNumberTable" => {
                let line_number_table_length = c.read_u16::<BE>()? as usize;
                let mut table = Vec::with_capacity(line_number_table_length);

                for _ in 0..line_number_table_length {
                    table.push(LineNumber::read_from(&mut c)?);
                }

                AttributeValue::LineNumberTable { table }
            }
            "LocalVariableTable" => {
                let local_variable_table_length = c.read_u16::<BE>()? as usize;
                let mut table = Vec::with_capacity(local_variable_table_length);

                for _ in 0..local_variable_table_length {
                    table.push(LocalVariable::read_from(&mut c)?);
                }

                AttributeValue::LocalVariableTable { table }
            }
            "SourceFile" => AttributeValue::SourceFile {
                value: c.read_u16::<BE>()? as usize,
            },
            "Synthetic" => AttributeValue::Synthetic,
            "AnnotationDefault" => AttributeValue::AnnotationDefault {
                default: data.to_vec(),
            },
            "EnclosingMethod" => AttributeValue::EnclosingMethod {
                class_index: c.read_u16::<BE>()? as usize,
                method_index: c.read_u16::<BE>()? as usize,
            },
            "LocalVariableTypeTable" => {
                let local_variable_type_table_length = c.read_u16::<BE>()? as usize;
                let mut table = Vec::with_capacity(local_variable_type_table_length);

                for _ in 0..local_variable_type_table_length {
                    table.push(LocalVariableType::read_from(&mut c)?);
                }

                AttributeValue::LocalVariableTypeTable { table }
            }
            _ => AttributeValue::Unknown(data.to_vec()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

impl Attribute {
    pub fn read_from<R: Read>(
        r: &mut R,
        constant_pool: &ConstantPool,
    ) -> Result<Attribute, AttributeReadError> {
        let name_i = r.read_u16::<BE>()? as usize;
        let name = match constant_pool.get(&name_i) {
            Some(c) => match c {
                Constant::Utf8 { value } => value.clone(),
                _ => return Err(AttributeReadError::InvalidNameType),
            },
            None => return Err(AttributeReadError::NoAttribName),
        };

        let length = r.read_u32::<BE>()? as usize;
        let data = r.read_byte_vec(length)?;

        Ok(Attribute {
            name: name.clone(),
            value: AttributeValue::from_name(name.as_str(), data.as_slice(), &constant_pool)?,
        })
    }
}
