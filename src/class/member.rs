use crate::class::access_flags::AccessFlags;
use crate::class::attribute::Attribute;
use crate::class::constant::{Constant, ConstantPool};
use crate::error::MemberReadError;
use byteorder::{ReadBytesExt, BE};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Member {
    pub access_flags: AccessFlags,
    pub name: String,
    pub descriptor: String, // TODO: Specialize
    pub attributes: Vec<Attribute>,
}

impl Member {
    pub fn read_from<R: Read>(
        r: &mut R,
        constant_pool: &ConstantPool,
    ) -> Result<Member, MemberReadError> {
        let access_flags = AccessFlags::read_from(r)?;

        let name_i = r.read_u16::<BE>()? as usize;
        let name = match constant_pool.get(&name_i) {
            Some(c) => match c {
                Constant::Utf8 { value } => value.clone(),
                _ => return Err(MemberReadError::InvalidNameType),
            },
            None => return Err(MemberReadError::NoMemberName),
        };

        let desc_i = r.read_u16::<BE>()? as usize;
        let descriptor = match constant_pool.get(&desc_i) {
            Some(c) => match c {
                Constant::Utf8 { value } => value.clone(),
                _ => return Err(MemberReadError::InvalidDescType),
            },
            None => return Err(MemberReadError::NoMemberDesc),
        };

        let attrib_count = r.read_u16::<BE>()? as usize;
        let mut attributes = Vec::with_capacity(attrib_count);

        for _ in 0..attrib_count {
            attributes.push(Attribute::read_from(r, constant_pool)?);
        }

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
