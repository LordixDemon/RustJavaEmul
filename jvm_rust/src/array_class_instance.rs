use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::{
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    mem,
};

use parking_lot::RwLock;

use jvm::{ArrayClassDefinition, ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut, ClassDefinition, ClassInstance, JavaType, JavaValue, Result};

use crate::{array_class_definition::ArrayClassDefinitionImpl, profile};

enum ArrayElements {
    I32(UnsafeCell<Vec<i32>>),
    I64(UnsafeCell<Vec<i64>>),
    F32(UnsafeCell<Vec<f32>>),
    F64(UnsafeCell<Vec<f64>>),
    U8(UnsafeCell<Vec<u8>>),
    U16(UnsafeCell<Vec<u16>>),
    Objects(RwLock<Vec<JavaValue>>),
}

unsafe impl Sync for ArrayElements {}

struct ArrayClassInstanceInner {
    class: Box<dyn ClassDefinition>,
    length: usize,
    element_type: JavaType,
    elements: ArrayElements,
}

#[derive(Clone)]
pub struct ArrayClassInstanceImpl {
    inner: Arc<ArrayClassInstanceInner>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassDefinitionImpl, length: usize) -> Self {
        profile::array_new();

        let element_type = JavaType::parse(&class.element_type_name());
        let elements = match &element_type {
            JavaType::Int => ArrayElements::I32(UnsafeCell::new(vec![0; length])),
            JavaType::Long => ArrayElements::I64(UnsafeCell::new(vec![0; length])),
            JavaType::Float => ArrayElements::F32(UnsafeCell::new(vec![0.0; length])),
            JavaType::Double => ArrayElements::F64(UnsafeCell::new(vec![0.0; length])),
            JavaType::Boolean | JavaType::Byte => ArrayElements::U8(UnsafeCell::new(vec![0; length])),
            JavaType::Char | JavaType::Short => ArrayElements::U16(UnsafeCell::new(vec![0; length])),
            JavaType::Class(_) | JavaType::Array(_) => {
                let default_value = element_type.default();
                ArrayElements::Objects(RwLock::new(vec![default_value; length]))
            }
            JavaType::Void | JavaType::Method(_, _) => unreachable!(),
        };

        Self {
            inner: Arc::new(ArrayClassInstanceInner {
                class: Box::new(class.clone()),
                length,
                element_type,
                elements,
            }),
        }
    }

    fn profile_array_type_index(element_type: &JavaType) -> usize {
        match element_type {
            JavaType::Boolean => 0,
            JavaType::Byte => 1,
            JavaType::Char => 2,
            JavaType::Short => 3,
            JavaType::Int => 4,
            JavaType::Long => 5,
            JavaType::Float => 6,
            JavaType::Double => 7,
            JavaType::Class(_) | JavaType::Array(_) | JavaType::Method(_, _) | JavaType::Void => 8,
        }
    }

    pub(crate) fn load_one_stack(&self, offset: usize) -> JavaValue {
        profile::array_load_one_typed(Self::profile_array_type_index(&self.inner.element_type));
        Self::to_stack_value(self.load_at(offset))
    }

    pub(crate) fn store_one_stack(&mut self, offset: usize, value: JavaValue) {
        profile::array_store_one_typed(Self::profile_array_type_index(&self.inner.element_type));
        self.store_at(offset, value);
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.length
    }

    pub fn i32_ptr_mut(&self) -> Option<(*mut i32, usize)> {
        match &self.inner.elements {
            ArrayElements::I32(values) => {
                let values = unsafe { &mut *values.get() };
                Some((values.as_mut_ptr(), values.len()))
            }
            _ => None,
        }
    }

    fn load_at(&self, offset: usize) -> JavaValue {
        match &self.inner.elements {
            ArrayElements::I32(values) => JavaValue::Int(unsafe { &*values.get() }[offset]),
            ArrayElements::I64(values) => JavaValue::Long(unsafe { &*values.get() }[offset]),
            ArrayElements::F32(values) => JavaValue::Float(unsafe { &*values.get() }[offset]),
            ArrayElements::F64(values) => JavaValue::Double(unsafe { &*values.get() }[offset]),
            ArrayElements::U8(values) => {
                let value = unsafe { &*values.get() }[offset];
                match self.inner.element_type {
                    JavaType::Boolean => JavaValue::Boolean(value != 0),
                    _ => JavaValue::Byte(value as i8),
                }
            }
            ArrayElements::U16(values) => {
                let value = unsafe { &*values.get() }[offset];
                match self.inner.element_type {
                    JavaType::Char => JavaValue::Char(value),
                    _ => JavaValue::Short(value as i16),
                }
            }
            ArrayElements::Objects(values) => values.read()[offset].clone(),
        }
    }

    fn store_at(&self, offset: usize, value: JavaValue) {
        match &self.inner.elements {
            ArrayElements::I32(values) => {
                let dest = unsafe { &mut *values.get() };
                dest[offset] = int_value(value);
            }
            ArrayElements::I64(values) => {
                let dest = unsafe { &mut *values.get() };
                dest[offset] = value.into();
            }
            ArrayElements::F32(values) => {
                let dest = unsafe { &mut *values.get() };
                dest[offset] = value.into();
            }
            ArrayElements::F64(values) => {
                let dest = unsafe { &mut *values.get() };
                dest[offset] = value.into();
            }
            ArrayElements::U8(values) => {
                let stored = if matches!(self.inner.element_type, JavaType::Boolean) {
                    (int_value(value) & 1) as u8
                } else {
                    int_value(value) as i8 as u8
                };
                let dest = unsafe { &mut *values.get() };
                dest[offset] = stored;
            }
            ArrayElements::U16(values) => {
                let dest = unsafe { &mut *values.get() };
                dest[offset] = int_value(value) as u16;
            }
            ArrayElements::Objects(values) => values.write()[offset] = value,
        }
    }

    fn to_stack_value(value: JavaValue) -> JavaValue {
        match value {
            JavaValue::Boolean(value) => JavaValue::Int(i32::from(value)),
            JavaValue::Byte(value) => JavaValue::Int(i32::from(value)),
            JavaValue::Char(value) => JavaValue::Int(i32::from(value)),
            JavaValue::Short(value) => JavaValue::Int(i32::from(value)),
            other => other,
        }
    }
}

#[async_trait::async_trait]
impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.inner.class.clone()
    }

    fn destroy(self: Box<Self>) {}

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ArrayClassInstanceImpl>();
        if other.is_none() {
            return Ok(false);
        }
        let other = other.unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()> {
        profile::array_store_bulk();

        match &self.inner.elements {
            ArrayElements::I32(storage) => {
                let dest = unsafe { &mut *storage.get() };
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = int_value(value);
                }
            }
            ArrayElements::I64(storage) => {
                let dest = unsafe { &mut *storage.get() };
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = value.into();
                }
            }
            ArrayElements::F32(storage) => {
                let dest = unsafe { &mut *storage.get() };
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = value.into();
                }
            }
            ArrayElements::F64(storage) => {
                let dest = unsafe { &mut *storage.get() };
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = value.into();
                }
            }
            ArrayElements::U8(storage) => {
                let dest = unsafe { &mut *storage.get() };
                let boolean = matches!(self.inner.element_type, JavaType::Boolean);
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = if boolean {
                        (int_value(value) & 1) as u8
                    } else {
                        int_value(value) as i8 as u8
                    };
                }
            }
            ArrayElements::U16(storage) => {
                let dest = unsafe { &mut *storage.get() };
                for (index, value) in values.into_vec().into_iter().enumerate() {
                    dest[offset + index] = int_value(value) as u16;
                }
            }
            ArrayElements::Objects(storage) => {
                storage.write().splice(offset..offset + values.len(), values.into_vec());
            }
        }

        Ok(())
    }

    fn store_one(&mut self, offset: usize, value: JavaValue) -> Result<()> {
        profile::array_store_one_typed(Self::profile_array_type_index(&self.inner.element_type));
        self.store_at(offset, value);
        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> Result<Vec<JavaValue>> {
        profile::array_load_bulk();

        let mut values = Vec::with_capacity(length);
        for index in 0..length {
            values.push(self.load_at(offset + index));
        }
        Ok(values)
    }

    fn load_one(&self, offset: usize) -> Result<JavaValue> {
        profile::array_load_one_typed(Self::profile_array_type_index(&self.inner.element_type));
        Ok(self.load_at(offset))
    }

    fn copy_to(&self, src_pos: usize, dest: &mut dyn ArrayClassInstance, dest_pos: usize, length: usize) -> Result<bool> {
        profile::array_copy();

        let Some(dest) = dest.as_any_mut().downcast_mut::<ArrayClassInstanceImpl>() else {
            return Ok(false);
        };

        if self.inner.element_type != dest.inner.element_type {
            return Ok(false);
        }

        if Arc::ptr_eq(&self.inner, &dest.inner) {
            copy_elements_overlapping(&self.inner.elements, src_pos, dest_pos, length);
            return Ok(true);
        }

        copy_elements(&self.inner.elements, &dest.inner.elements, src_pos, dest_pos, length);
        Ok(true)
    }

    fn raw_buffer(&self) -> Result<Box<dyn ArrayRawBuffer>> {
        Ok(Box::new(ArrayRawBufferImpl { inner: self.inner.clone() }))
    }

    fn raw_buffer_mut(&mut self) -> Result<Box<dyn ArrayRawBufferMut>> {
        Ok(Box::new(ArrayRawBufferImpl { inner: self.inner.clone() }))
    }

    fn length(&self) -> usize {
        self.inner.length
    }

    fn is_object_array(&self) -> bool {
        matches!(self.inner.elements, ArrayElements::Objects(_))
    }

    fn i32_slice(&self) -> Option<&[i32]> {
        match &self.inner.elements {
            ArrayElements::I32(values) => Some(unsafe { &*values.get() }.as_slice()),
            _ => None,
        }
    }

    fn i32_slice_mut(&mut self) -> Option<&mut [i32]> {
        match &self.inner.elements {
            ArrayElements::I32(values) => Some(unsafe { &mut *values.get() }.as_mut_slice()),
            _ => None,
        }
    }
}

fn int_value(value: JavaValue) -> i32 {
    match value {
        JavaValue::Boolean(value) => value as i32,
        JavaValue::Byte(value) => value as i32,
        JavaValue::Char(value) => value as i32,
        JavaValue::Short(value) => value as i32,
        JavaValue::Int(value) => value,
        _ => panic!("Expected integer-compatible value, got {value:?}"),
    }
}

fn copy_elements_overlapping(elements: &ArrayElements, src_pos: usize, dest_pos: usize, length: usize) {
    match elements {
        ArrayElements::I32(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::I64(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::F32(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::F64(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::U8(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::U16(values) => unsafe { &mut *values.get() }.copy_within(src_pos..src_pos + length, dest_pos),
        ArrayElements::Objects(values) => {
            let mut values = values.write();
            let copied = values[src_pos..src_pos + length].to_vec();
            values.splice(dest_pos..dest_pos + length, copied);
        }
    }
}

fn copy_range<T: Copy>(src: &UnsafeCell<Vec<T>>, dest: &UnsafeCell<Vec<T>>, src_pos: usize, dest_pos: usize, length: usize) {
    let src = unsafe { &*src.get() };
    let dest = unsafe { &mut *dest.get() };
    dest[dest_pos..dest_pos + length].copy_from_slice(&src[src_pos..src_pos + length]);
}

fn copy_elements(src: &ArrayElements, dest: &ArrayElements, src_pos: usize, dest_pos: usize, length: usize) {
    match (src, dest) {
        (ArrayElements::I32(src), ArrayElements::I32(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::I64(src), ArrayElements::I64(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::F32(src), ArrayElements::F32(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::F64(src), ArrayElements::F64(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::U8(src), ArrayElements::U8(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::U16(src), ArrayElements::U16(dest)) => copy_range(src, dest, src_pos, dest_pos, length),
        (ArrayElements::Objects(src), ArrayElements::Objects(dest)) => {
            let copied = src.read()[src_pos..src_pos + length].to_vec();
            dest.write().splice(dest_pos..dest_pos + length, copied);
        }
        _ => {}
    }
}

fn as_bytes<T>(values: &[T]) -> &[u8] {
    unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<u8>(), mem::size_of_val(values)) }
}

fn as_bytes_mut<T>(values: &mut [T]) -> &mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast::<u8>(), mem::size_of_val(values)) }
}

impl Hash for ArrayClassInstanceImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl Debug for ArrayClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayClassInstance({})", self.inner.class.name())
    }
}

struct ArrayRawBufferImpl {
    inner: Arc<ArrayClassInstanceInner>,
}

impl ArrayRawBufferImpl {
    fn primitive_bytes(&self) -> Option<&[u8]> {
        Some(match &self.inner.elements {
            ArrayElements::I32(values) => as_bytes(unsafe { &*values.get() }.as_slice()),
            ArrayElements::I64(values) => as_bytes(unsafe { &*values.get() }.as_slice()),
            ArrayElements::F32(values) => as_bytes(unsafe { &*values.get() }.as_slice()),
            ArrayElements::F64(values) => as_bytes(unsafe { &*values.get() }.as_slice()),
            ArrayElements::U8(values) => unsafe { &*values.get() }.as_slice(),
            ArrayElements::U16(values) => as_bytes(unsafe { &*values.get() }.as_slice()),
            ArrayElements::Objects(_) => return None,
        })
    }

    fn primitive_bytes_mut(&mut self) -> Option<&mut [u8]> {
        Some(match &self.inner.elements {
            ArrayElements::I32(values) => as_bytes_mut(unsafe { &mut *values.get() }.as_mut_slice()),
            ArrayElements::I64(values) => as_bytes_mut(unsafe { &mut *values.get() }.as_mut_slice()),
            ArrayElements::F32(values) => as_bytes_mut(unsafe { &mut *values.get() }.as_mut_slice()),
            ArrayElements::F64(values) => as_bytes_mut(unsafe { &mut *values.get() }.as_mut_slice()),
            ArrayElements::U8(values) => unsafe { &mut *values.get() }.as_mut_slice(),
            ArrayElements::U16(values) => as_bytes_mut(unsafe { &mut *values.get() }.as_mut_slice()),
            ArrayElements::Objects(_) => return None,
        })
    }

    fn element_size(&self) -> usize {
        match &self.inner.elements {
            ArrayElements::I32(_) | ArrayElements::F32(_) => 4,
            ArrayElements::I64(_) | ArrayElements::F64(_) => 8,
            ArrayElements::U8(_) => 1,
            ArrayElements::U16(_) => 2,
            ArrayElements::Objects(_) => 0,
        }
    }
}

impl ArrayRawBuffer for ArrayRawBufferImpl {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        profile::array_raw_read();

        let Some(bytes) = self.primitive_bytes() else {
            panic!("Expected primitive array");
        };
        let byte_offset = offset * self.element_size();
        let count = buffer.len().min(bytes.len().saturating_sub(byte_offset));
        if count > 0 {
            buffer[..count].copy_from_slice(&bytes[byte_offset..byte_offset + count]);
        }

        Ok(())
    }

    fn i32_slice(&self) -> Option<&[i32]> {
        match &self.inner.elements {
            ArrayElements::I32(values) => Some(unsafe { &*values.get() }.as_slice()),
            _ => None,
        }
    }
}

impl ArrayRawBufferMut for ArrayRawBufferImpl {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()> {
        profile::array_raw_write();

        let element_size = self.element_size();
        let Some(bytes) = self.primitive_bytes_mut() else {
            panic!("Expected primitive array");
        };
        let byte_offset = offset * element_size;
        let count = buffer.len().min(bytes.len().saturating_sub(byte_offset));
        if count > 0 {
            bytes[byte_offset..byte_offset + count].copy_from_slice(&buffer[..count]);
        }

        Ok(())
    }

    fn i32_slice_mut(&mut self) -> Option<&mut [i32]> {
        match &self.inner.elements {
            ArrayElements::I32(values) => Some(unsafe { &mut *values.get() }.as_mut_slice()),
            _ => None,
        }
    }
}
