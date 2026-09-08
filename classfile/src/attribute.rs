use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::flat_map,
    error::{Error, ErrorKind},
    multi::length_count,
    number::complete::{be_u16, be_u32},
};

use crate::{ConstantPoolReference, constant_pool::ConstantPoolItem, opcode::Opcode};

pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<Arc<String>>,
}

impl CodeAttributeExceptionTable {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        let (data, start_pc) = be_u16(data)?;
        let (data, end_pc) = be_u16(data)?;
        let (data, handler_pc) = be_u16(data)?;
        let (data, catch_type) = be_u16(data)?;
        let catch_type = if catch_type != 0 {
            Some(crate::constant_pool::resolve_class_name(data, constant_pool, catch_type)?.1)
        } else {
            None
        };

        Ok((
            data,
            Self {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            },
        ))
    }
}

pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: BTreeMap<u32, Opcode>, // TODO we can store it Vec<u8> and create code iterator..
    pub code_offsets: Vec<u32>,
    pub code_sequence: Vec<(u32, Opcode)>,
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        let (data, max_stack) = be_u16(data)?;
        let (data, max_locals) = be_u16(data)?;
        let (data, code_bytes): (_, &[u8]) = flat_map(be_u32, take).parse(data)?;
        let code = Self::parse_code(code_bytes, constant_pool);
        let (data, exception_table) = length_count(be_u16, |x| CodeAttributeExceptionTable::parse(x, constant_pool)).parse(data)?;
        let (data, attributes) = length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)).parse(data)?;
        let code_offsets = code.keys().copied().collect();
        let code_sequence = code.iter().map(|(offset, opcode)| (*offset, opcode.clone())).collect();
        Ok((
            data,
            Self {
                max_stack,
                max_locals,
                code,
                code_offsets,
                code_sequence,
                exception_table,
                attributes,
            },
        ))
    }

    fn parse_code(code: &[u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> BTreeMap<u32, Opcode> {
        let mut result = BTreeMap::new();

        let mut data = code;
        loop {
            let offset = unsafe { data.as_ptr().offset_from(code.as_ptr()) } as usize;
            if let Ok((remaining, opcode)) = Opcode::parse(data, offset, constant_pool) {
                result.insert(offset as _, opcode);

                data = remaining;
            } else {
                break;
            }
        }

        result
    }
}

pub struct AttributeInfoLineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

impl AttributeInfoLineNumberTableEntry {
    pub fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, start_pc) = be_u16(data)?;
        let (data, line_number) = be_u16(data)?;

        Ok((data, Self { start_pc, line_number }))
    }
}

pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub index: u16,
}

impl LocalVariableTableEntry {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        let (data, start_pc) = be_u16(data)?;
        let (data, length) = be_u16(data)?;
        let (data, name) = crate::constant_pool::parse_utf8_index(data, constant_pool)?;
        let (data, descriptor) = crate::constant_pool::parse_utf8_index(data, constant_pool)?;
        let (data, index) = be_u16(data)?;
        Ok((
            data,
            Self {
                start_pc,
                length,
                name,
                descriptor,
                index,
            },
        ))
    }
}

pub enum AttributeInfo {
    ConstantValue(ConstantPoolReference),
    Code(AttributeInfoCode),
    StackMap(Vec<u8>),      // TODO Older variant of StackMapTable
    StackMapTable(Vec<u8>), // TODO
    Exceptions(Vec<u8>),    // TODO
    InnerClasses(Vec<u8>),  // TODO
    Synthetic(Vec<u8>),     // TODO
    SourceFile(Arc<String>),
    SourceDebugExtension,
    LineNumberTable(Vec<AttributeInfoLineNumberTableEntry>),
    LocalVariableTable(Vec<LocalVariableTableEntry>),
    BootstrapMethods(Vec<u8>), // TODO
    MethodParameters(Vec<u8>), // TODO
    NestMembers(Vec<u8>),      // TODO
    NestHost(Vec<u8>),         // TODO
    Other(Vec<u8>),
}

impl AttributeInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        let (data, name) = crate::constant_pool::parse_utf8_index(data, constant_pool)?;
        let (data, info): (_, &[u8]) = flat_map(be_u32, take).parse(data)?;
        let attr = match name.as_str() {
            "ConstantValue" => AttributeInfo::ConstantValue(Self::parse_constant_value(info, constant_pool)?.1),
            "Code" => AttributeInfo::Code(AttributeInfoCode::parse(info, constant_pool)?.1),
            "LineNumberTable" => AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse).parse(info)?.1),
            "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(info, constant_pool)?.1),
            "LocalVariableTable" => AttributeInfo::LocalVariableTable(Self::parse_local_variable_table(info, constant_pool)?.1),
            "StackMap" => AttributeInfo::StackMap(info.to_vec()),
            "StackMapTable" => AttributeInfo::StackMapTable(info.to_vec()),
            "Exceptions" => AttributeInfo::Exceptions(info.to_vec()),
            "InnerClasses" => AttributeInfo::InnerClasses(info.to_vec()),
            "Synthetic" => AttributeInfo::Synthetic(info.to_vec()),
            "BootstrapMethods" => AttributeInfo::BootstrapMethods(info.to_vec()),
            "MethodParameters" => AttributeInfo::MethodParameters(info.to_vec()),
            "NestMembers" => AttributeInfo::NestMembers(info.to_vec()),
            "NestHost" => AttributeInfo::NestHost(info.to_vec()),
            _ => AttributeInfo::Other(info.to_vec()),
        };
        Ok((data, attr))
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
        crate::constant_pool::parse_utf8_index(data, constant_pool)
    }

    fn parse_constant_value<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], ConstantPoolReference> {
        let (data, index) = be_u16(data)?;
        match ConstantPoolReference::try_from_constant_pool(constant_pool, index) {
            Some(value) => Ok((data, value)),
            None => Err(nom::Err::Error(Error::new(data, ErrorKind::Verify))),
        }
    }

    fn parse_local_variable_table<'a>(
        data: &'a [u8],
        constant_pool: &BTreeMap<u16, ConstantPoolItem>,
    ) -> IResult<&'a [u8], Vec<LocalVariableTableEntry>> {
        length_count(be_u16, |x| LocalVariableTableEntry::parse(x, constant_pool)).parse(data)
    }
}
