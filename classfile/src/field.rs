use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{IResult, Parser, multi::length_count, number::complete::be_u16};

use java_constants::FieldAccessFlags;

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem};

pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        let (data, access_flags) = be_u16(data)?;
        let (data, name) = crate::constant_pool::parse_utf8_index(data, constant_pool)?;
        let (data, descriptor) = crate::constant_pool::parse_utf8_index(data, constant_pool)?;
        let (data, attributes) = length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)).parse(data)?;
        Ok((
            data,
            Self {
                access_flags: FieldAccessFlags::from_bits_truncate(access_flags),
                name,
                descriptor,
                attributes,
            },
        ))
    }
}
