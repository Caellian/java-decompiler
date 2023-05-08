use crate::access_flags::AccessFlags;
use crate::attribute::AttributeValue;
use crate::constant::{Constant, ConstantPool};
use crate::error::MemberError;
use crate::Descriptor;
use byteorder::{ReadBytesExt, BE};
use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Member {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: Descriptor, // TODO: Specialize
    pub attributes: HashMap<String, AttributeValue>,
}

impl Member {
    pub fn read_from<R: Read>(
        r: &mut R,
        constant_pool: &ConstantPool,
    ) -> Result<Member, MemberError> {
        let access_flags = AccessFlags::read_from(r)?;

        let name_i = r.read_u16::<BE>()?;
        let name = match constant_pool.get(&name_i) {
            Some(c) => match c {
                Constant::Utf8 { value } => value.clone(),
                _ => return Err(MemberError::InvalidNameType),
            },
            None => return Err(MemberError::NoMemberName),
        };

        let desc_i = r.read_u16::<BE>()?;
        let descriptor = match constant_pool.get(&desc_i) {
            Some(c) => match c {
                Constant::Utf8 { value } => Descriptor::from_str(&value).map_err(MemberError::from),
                _ => Err(MemberError::InvalidDesc),
            },
            None => Err(MemberError::NoMemberDesc),
        }?;

        let attributes = AttributeValue::read_all(r, Some(constant_pool))?;

        Ok(Member {
            access_flags,
            name,
            descriptor,
            attributes,
        })
    }

    pub fn is_constructor(&self) -> bool {
        self.name == "<init>"
    }
}
