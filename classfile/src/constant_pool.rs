use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    error::{Error, ErrorKind},
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, u8},
};

fn decode_modified_utf8(bytes: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == 0 {
            out.push('\0');
            i += 1;
        } else if b < 0x80 {
            out.push(b as char);
            i += 1;
        } else if b & 0xE0 == 0xC0 && i + 1 < bytes.len() {
            let code = ((b as u32 & 0x1F) << 6) | (bytes[i + 1] as u32 & 0x3F);
            out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
            i += 2;
        } else if b & 0xF0 == 0xE0 && i + 2 < bytes.len() {
            let code = ((b as u32 & 0x0F) << 12) | ((bytes[i + 1] as u32 & 0x3F) << 6) | (bytes[i + 2] as u32 & 0x3F);
            out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
            i += 3;
        } else {
            out.push('\u{FFFD}');
            i += 1;
        }
    }
    out
}

fn parse_utf8(data: &[u8]) -> IResult<&[u8], Arc<String>> {
    let (data, length) = be_u16(data)?;
    let (data, utf8) = take(length as usize).parse(data)?;

    Ok((data, Arc::new(decode_modified_utf8(utf8))))
}

#[derive(Debug)]
pub enum ConstantPoolItem {
    Utf8(Arc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class { name_index: u16 },
    String { string_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    Unsupported,
}

impl ConstantPoolItem {
    fn parse_tagged(data: &[u8], tag: u8) -> IResult<&[u8], Self> {
        match tag {
            1 => {
                let (data, utf8) = parse_utf8(data)?;
                Ok((data, Self::Utf8(utf8)))
            }
            3 => {
                let (data, value) = be_i32(data)?;
                Ok((data, Self::Integer(value)))
            }
            4 => {
                let (data, value) = be_f32(data)?;
                Ok((data, Self::Float(value)))
            }
            5 => {
                let (data, value) = be_i64(data)?;
                Ok((data, Self::Long(value)))
            }
            6 => {
                let (data, value) = be_f64(data)?;
                Ok((data, Self::Double(value)))
            }
            7 => {
                let (data, name_index) = be_u16(data)?;
                Ok((data, Self::Class { name_index }))
            }
            8 => {
                let (data, string_index) = be_u16(data)?;
                Ok((data, Self::String { string_index }))
            }
            9 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Fieldref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            10 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Methodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            11 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            12 => {
                let (data, name_index) = be_u16(data)?;
                let (data, descriptor_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::NameAndType {
                        name_index,
                        descriptor_index,
                    },
                ))
            }
            15 => {
                let (data, _) = u8(data)?;
                let (data, _) = be_u16(data)?;
                Ok((data, Self::Unsupported))
            }
            16 | 19 | 20 => {
                let (data, _) = be_u16(data)?;
                Ok((data, Self::Unsupported))
            }
            17 | 18 => {
                let (data, _) = be_u16(data)?;
                let (data, _) = be_u16(data)?;
                Ok((data, Self::Unsupported))
            }
            _ => Err(nom::Err::Error(Error::new(data, ErrorKind::Switch))),
        }
    }

    pub fn parse_all(data: &[u8]) -> IResult<&[u8], BTreeMap<u16, Self>> {
        let (remaining, count) = be_u16(data)?;

        let mut data = remaining;
        let mut result = BTreeMap::new();
        let mut i = 1;
        loop {
            let (remaining, item) = Self::parse_with_tag(data)?;
            let is_double_entry = match &item {
                Self::Long(_) | Self::Double(_) => {
                    // long or double constant takes two constant pool entries....
                    true
                }
                _ => false,
            };
            result.insert(i, item);

            data = remaining;
            i += 1;
            if is_double_entry {
                i += 1;
            }

            if i >= count {
                break;
            }
        }

        Ok((data, result))
    }

    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, tag) = u8(data)?;
        Self::parse_tagged(data, tag)
    }

    pub fn utf8(&self) -> Option<Arc<String>> {
        if let ConstantPoolItem::Utf8(x) = self { Some(x.clone()) } else { None }
    }

    pub fn class_name_index(&self) -> Option<u16> {
        if let ConstantPoolItem::Class { name_index } = self {
            Some(*name_index)
        } else {
            None
        }
    }

    pub fn name_and_type(&self) -> Option<(u16, u16)> {
        if let ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } = self
        {
            Some((*name_index, *descriptor_index))
        } else {
            None
        }
    }
}

fn nom_verify<T>(input: &[u8], value: Option<T>) -> IResult<&[u8], T> {
    match value {
        Some(value) => Ok((input, value)),
        None => Err(nom::Err::Error(Error::new(input, ErrorKind::Verify))),
    }
}

pub(crate) fn utf8_at(pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Arc<String>> {
    pool.get(&index).and_then(ConstantPoolItem::utf8)
}

pub(crate) fn class_name_at(pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Arc<String>> {
    let name_index = pool.get(&index).and_then(ConstantPoolItem::class_name_index)?;
    utf8_at(pool, name_index)
}

pub(crate) fn parse_utf8_index<'a>(data: &'a [u8], pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    let (data, index) = be_u16(data)?;
    nom_verify(data, utf8_at(pool, index))
}

pub(crate) fn parse_class_name_index<'a>(data: &'a [u8], pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    let (data, index) = be_u16(data)?;
    nom_verify(data, class_name_at(pool, index))
}

pub(crate) fn resolve_class_name<'a>(data: &'a [u8], pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> IResult<&'a [u8], Arc<String>> {
    nom_verify(data, class_name_at(pool, index))
}

#[derive(Clone, Debug)]
pub enum ConstantPoolReference {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(Arc<String>),
    Class(Arc<String>),
    Method(FieldMethodref),
    InterfaceMethodref(FieldMethodref),
    Field(FieldMethodref),
}

impl ConstantPoolReference {
    pub fn try_from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Self> {
        Some(match constant_pool.get(&index)? {
            ConstantPoolItem::Integer(x) => Self::Integer(*x),
            ConstantPoolItem::Float(x) => Self::Float(*x),
            ConstantPoolItem::Long(x) => Self::Long(*x),
            ConstantPoolItem::Double(x) => Self::Double(*x),
            ConstantPoolItem::String { string_index } => Self::String(utf8_at(constant_pool, *string_index)?),
            ConstantPoolItem::Class { name_index } => Self::Class(utf8_at(constant_pool, *name_index)?),
            ConstantPoolItem::Utf8(x) => Self::String(x.clone()),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Self::Method(FieldMethodref::try_from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?),
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Self::Field(FieldMethodref::try_from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?),
            ConstantPoolItem::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => Self::InterfaceMethodref(FieldMethodref::try_from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?),
            ConstantPoolItem::NameAndType { .. } | ConstantPoolItem::Unsupported => return None,
        })
    }

    pub fn from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Self {
        Self::try_from_constant_pool(constant_pool, index).expect("invalid constant pool reference")
    }

    pub fn try_as_class(&self) -> Option<&str> {
        if let Self::Class(x) = self { Some(x) } else { None }
    }

    pub fn as_class(&self) -> &str {
        self.try_as_class().expect("invalid constant pool item")
    }

    pub fn try_as_field_ref(&self) -> Option<&FieldMethodref> {
        if let Self::Field(x) = self { Some(x) } else { None }
    }

    pub fn as_field_ref(&self) -> &FieldMethodref {
        self.try_as_field_ref().expect("invalid constant pool item")
    }

    pub fn try_as_method_ref(&self) -> Option<&FieldMethodref> {
        if let Self::Method(x) = self { Some(x) } else { None }
    }

    pub fn as_method_ref(&self) -> &FieldMethodref {
        self.try_as_method_ref().expect("invalid constant pool item")
    }

    pub fn try_as_interface_method_ref(&self) -> Option<&FieldMethodref> {
        if let Self::InterfaceMethodref(x) = self { Some(x) } else { None }
    }

    pub fn as_interface_method_ref(&self) -> &FieldMethodref {
        self.try_as_interface_method_ref().expect("invalid constant pool item")
    }
}

#[derive(Clone, Debug)]
pub enum MethodParamKind {
    Boolean,
    Byte,
    Char,
    Short,
    Other,
}

#[derive(Clone, Debug)]
pub struct FieldMethodref {
    pub class: Arc<String>,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub method_param_kinds: Vec<MethodParamKind>,
}

impl FieldMethodref {
    pub fn try_from_reference_info(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16) -> Option<Self> {
        let class_name_index = match constant_pool.get(&class_index)? {
            ConstantPoolItem::Class { name_index } => *name_index,
            _ => return None,
        };
        let class_name = match constant_pool.get(&class_name_index)? {
            ConstantPoolItem::Utf8(value) => value.clone(),
            _ => return None,
        };
        let (name_index, descriptor_index) = match constant_pool.get(&name_and_type_index)? {
            ConstantPoolItem::NameAndType {
                name_index,
                descriptor_index,
            } => (*name_index, *descriptor_index),
            _ => return None,
        };
        let name = match constant_pool.get(&name_index)? {
            ConstantPoolItem::Utf8(value) => value.clone(),
            _ => return None,
        };
        let descriptor = match constant_pool.get(&descriptor_index)? {
            ConstantPoolItem::Utf8(value) => value.clone(),
            _ => return None,
        };
        let method_param_kinds = parse_method_param_kinds(&descriptor);
        Some(Self {
            class: class_name,
            name,
            descriptor,
            method_param_kinds,
        })
    }

    pub fn from_reference_info(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16) -> Self {
        Self::try_from_reference_info(constant_pool, class_index, name_and_type_index).expect("invalid constant pool Field/Methodref")
    }
}

fn parse_method_param_kinds(descriptor: &str) -> Vec<MethodParamKind> {
    let bytes = descriptor.as_bytes();
    if bytes.first() != Some(&b'(') {
        return Vec::new();
    }

    let mut kinds = Vec::new();
    let mut index = 1;

    while index < bytes.len() && bytes[index] != b')' {
        let kind = match bytes[index] {
            b'Z' => {
                index += 1;
                MethodParamKind::Boolean
            }
            b'B' => {
                index += 1;
                MethodParamKind::Byte
            }
            b'C' => {
                index += 1;
                MethodParamKind::Char
            }
            b'S' => {
                index += 1;
                MethodParamKind::Short
            }
            b'L' => {
                index += 1;
                while index < bytes.len() && bytes[index] != b';' {
                    index += 1;
                }
                if index < bytes.len() {
                    index += 1;
                }
                MethodParamKind::Other
            }
            b'[' => {
                while index < bytes.len() && bytes[index] == b'[' {
                    index += 1;
                }
                if index < bytes.len() && bytes[index] == b'L' {
                    index += 1;
                    while index < bytes.len() && bytes[index] != b';' {
                        index += 1;
                    }
                }
                if index < bytes.len() {
                    index += 1;
                }
                MethodParamKind::Other
            }
            _ => {
                index += 1;
                MethodParamKind::Other
            }
        };
        kinds.push(kind);
    }

    kinds
}
