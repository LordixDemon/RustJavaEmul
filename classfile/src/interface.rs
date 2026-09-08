use alloc::{collections::BTreeMap, string::String, sync::Arc};

use nom::IResult;

use crate::constant_pool::ConstantPoolItem;

pub fn parse_interface<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    crate::constant_pool::parse_class_name_index(data, constant_pool)
}
