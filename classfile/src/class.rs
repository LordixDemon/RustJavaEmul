use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    multi::length_count,
    number::complete::{be_u16, be_u32},
};

use java_constants::ClassAccessFlags;

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem, field::FieldInfo, interface::parse_interface, method::MethodInfo};

type ConstantParse = (BTreeMap<u16, ConstantPoolItem>, Option<Arc<String>>);

fn parse_this_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    crate::constant_pool::parse_class_name_index(data, constant_pool)
}

fn parse_super_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Option<Arc<String>>> {
    let (data, super_class) = be_u16(data)?;
    if super_class == 0 {
        Ok((data, None))
    } else {
        let (data, name) = crate::constant_pool::resolve_class_name(data, constant_pool, super_class)?;
        Ok((data, Some(name)))
    }
}

pub struct ClassInfo {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: BTreeMap<u16, ConstantPoolItem>, // TODO change to Vec
    pub access_flags: ClassAccessFlags,
    pub this_class: Arc<String>,
    pub super_class: Option<Arc<String>>,
    pub interfaces: Vec<Arc<String>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassInfo {
    fn parse_info(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, magic) = be_u32(data)?;
        if magic != 0xCAFEBABE {
            return Err(nom::Err::Error(nom::error::Error::new(data, nom::error::ErrorKind::Verify)));
        }

        let (data, minor_version) = be_u16(data)?;
        let (data, major_version) = be_u16(data)?;
        let (data, constant_pool) = ConstantPoolItem::parse_all(data)?;
        let (data, access_flags) = be_u16(data)?;
        let (data, this_class) = parse_this_class(data, &constant_pool)?;
        let (data, super_class) = parse_super_class(data, &constant_pool)?;
        let (data, interfaces) = length_count(be_u16, |x| parse_interface(x, &constant_pool)).parse(data)?;
        let (data, fields) = length_count(be_u16, |x| FieldInfo::parse(x, &constant_pool)).parse(data)?;
        let (data, methods) = length_count(be_u16, |x| MethodInfo::parse(x, &constant_pool)).parse(data)?;
        let (data, attributes) = length_count(be_u16, |x| AttributeInfo::parse(x, &constant_pool)).parse(data)?;

        Ok((
            data,
            Self {
                magic,
                minor_version,
                major_version,
                constant_pool,
                access_flags: ClassAccessFlags::from_bits_truncate(access_flags),
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            },
        ))
    }

    pub fn parse(file: &[u8]) -> Option<Self> {
        Self::parse_info(file).ok().map(|(_, result)| result)
    }

    pub fn parse_constants(file: &[u8]) -> Option<ConstantParse> {
        Self::parse_constants_info(file).ok().map(|(_, result)| result)
    }

    fn parse_constants_info(file: &[u8]) -> IResult<&[u8], ConstantParse> {
        let (data, magic) = be_u32(file)?;
        if magic != 0xCAFEBABE {
            return Err(nom::Err::Error(nom::error::Error::new(data, nom::error::ErrorKind::Verify)));
        }
        let (data, _) = be_u16(data)?;
        let (data, _) = be_u16(data)?;
        let (data, constant_pool) = ConstantPoolItem::parse_all(data)?;
        let (data, _) = be_u16(data)?;
        let (data, this_class) = be_u16(data)?;
        let this_name = crate::constant_pool::class_name_at(&constant_pool, this_class);
        Ok((data, (constant_pool, this_name)))
    }
}
