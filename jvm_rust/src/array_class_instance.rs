use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU8, Ordering},
};

use parking_lot::RwLock;

use jvm::{ArrayClassDefinition, ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut, ClassDefinition, ClassInstance, JavaType, JavaValue, Result};

use crate::{array_class_definition::ArrayClassDefinitionImpl, profile};

enum ArrayElements {
    Primitive(Vec<AtomicU8>),
    NonPrimitive(RwLock<Vec<JavaValue>>),
}

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

        let elements = if matches!(element_type, JavaType::Class(_) | JavaType::Array(_)) {
            let default_value = element_type.default();
            ArrayElements::NonPrimitive(RwLock::new(vec![default_value; length]))
        } else {
            let element_size = Self::primitive_element_size(&element_type);
            ArrayElements::Primitive((0..length * element_size).map(|_| AtomicU8::new(0)).collect())
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

    fn primitive_element_size(element_type: &JavaType) -> usize {
        match element_type {
            JavaType::Boolean => 1,
            JavaType::Byte => 1,
            JavaType::Char => 2,
            JavaType::Short => 2,
            JavaType::Int => 4,
            JavaType::Long => 8,
            JavaType::Float => 4,
            JavaType::Double => 8,
            _ => unreachable!(),
        }
    }

    fn load_primitive_value(values: &[AtomicU8], element_type: &JavaType, byte_offset: usize) -> JavaValue {
        match element_type {
            JavaType::Boolean => JavaValue::Boolean(values[byte_offset].load(Ordering::Relaxed) != 0),
            JavaType::Byte => JavaValue::Byte(values[byte_offset].load(Ordering::Relaxed) as i8),
            JavaType::Char => JavaValue::Char(u16::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 2]))),
            JavaType::Short => JavaValue::Short(i16::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 2]))),
            JavaType::Int => JavaValue::Int(i32::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 4]))),
            JavaType::Long => JavaValue::Long(i64::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 8]))),
            JavaType::Float => JavaValue::Float(f32::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 4]))),
            JavaType::Double => JavaValue::Double(f64::from_le_bytes(load_bytes_array(&values[byte_offset..byte_offset + 8]))),
            _ => unreachable!(),
        }
    }

    fn store_primitive_value(values: &[AtomicU8], element_type: &JavaType, byte_offset: usize, value: JavaValue) {
        match element_type {
            JavaType::Boolean => {
                values[byte_offset].store((int_value(value) & 1 != 0) as u8, Ordering::Relaxed);
            }
            JavaType::Byte => {
                values[byte_offset].store(int_value(value) as i8 as u8, Ordering::Relaxed);
            }
            JavaType::Char => {
                store_bytes(&values[byte_offset..byte_offset + 2], &(int_value(value) as u16).to_le_bytes());
            }
            JavaType::Short => {
                store_bytes(&values[byte_offset..byte_offset + 2], &(int_value(value) as i16).to_le_bytes());
            }
            JavaType::Int => {
                store_bytes(&values[byte_offset..byte_offset + 4], &int_value(value).to_le_bytes());
            }
            JavaType::Long => {
                let value: i64 = value.into();
                store_bytes(&values[byte_offset..byte_offset + 8], &value.to_le_bytes());
            }
            JavaType::Float => {
                let value: f32 = value.into();
                store_bytes(&values[byte_offset..byte_offset + 4], &value.to_le_bytes());
            }
            JavaType::Double => {
                let value: f64 = value.into();
                store_bytes(&values[byte_offset..byte_offset + 8], &value.to_le_bytes());
            }
            _ => unreachable!(),
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
            JavaType::Class(_) | JavaType::Array(_) | JavaType::Method(_, _) => 8,
            JavaType::Void => 8,
        }
    }

    pub(crate) fn load_one_stack(&self, offset: usize) -> JavaValue {
        profile::array_load_one_typed(Self::profile_array_type_index(&self.inner.element_type));

        match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let offset = offset * Self::primitive_element_size(&self.inner.element_type);
                match self.inner.element_type {
                    JavaType::Boolean => JavaValue::Int((x[offset].load(Ordering::Relaxed) != 0) as i32),
                    JavaType::Byte => JavaValue::Int(x[offset].load(Ordering::Relaxed) as i8 as i32),
                    JavaType::Char => JavaValue::Int(u16::from_le_bytes(load_bytes_array(&x[offset..offset + 2])) as i32),
                    JavaType::Short => JavaValue::Int(i16::from_le_bytes(load_bytes_array(&x[offset..offset + 2])) as i32),
                    JavaType::Int => JavaValue::Int(i32::from_le_bytes(load_bytes_array(&x[offset..offset + 4]))),
                    JavaType::Long => JavaValue::Long(i64::from_le_bytes(load_bytes_array(&x[offset..offset + 8]))),
                    JavaType::Float => JavaValue::Float(f32::from_le_bytes(load_bytes_array(&x[offset..offset + 4]))),
                    JavaType::Double => JavaValue::Double(f64::from_le_bytes(load_bytes_array(&x[offset..offset + 8]))),
                    _ => unreachable!(),
                }
            }
            ArrayElements::NonPrimitive(x) => x.read()[offset].clone(),
        }
    }

    pub(crate) fn store_one_stack(&mut self, offset: usize, value: JavaValue) {
        profile::array_store_one_typed(Self::profile_array_type_index(&self.inner.element_type));

        match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let offset = offset * Self::primitive_element_size(&self.inner.element_type);
                Self::store_primitive_value(x, &self.inner.element_type, offset, value);
            }
            ArrayElements::NonPrimitive(x) => {
                x.write()[offset] = value;
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.length
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
            ArrayElements::Primitive(x) => {
                let element_size = Self::primitive_element_size(&self.inner.element_type);
                let start = offset * element_size;

                for (index, value) in values.into_vec().into_iter().enumerate() {
                    Self::store_primitive_value(x, &self.inner.element_type, start + index * element_size, value);
                }
            }
            ArrayElements::NonPrimitive(x) => {
                x.write().splice(offset..offset + values.len(), values.into_vec());
            }
        }

        Ok(())
    }

    fn store_one(&mut self, offset: usize, value: JavaValue) -> Result<()> {
        profile::array_store_one_typed(Self::profile_array_type_index(&self.inner.element_type));

        match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let offset = offset * Self::primitive_element_size(&self.inner.element_type);
                Self::store_primitive_value(x, &self.inner.element_type, offset, value);
            }
            ArrayElements::NonPrimitive(x) => {
                x.write()[offset] = value;
            }
        }

        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> Result<Vec<JavaValue>> {
        profile::array_load_bulk();

        Ok(match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let element_size = Self::primitive_element_size(&self.inner.element_type);
                let start = offset * element_size;
                let mut values = Vec::with_capacity(length);
                for index in 0..length {
                    values.push(Self::load_primitive_value(x, &self.inner.element_type, start + index * element_size));
                }

                values
            }
            ArrayElements::NonPrimitive(x) => x.read()[offset..offset + length].to_vec(),
        })
    }

    fn load_one(&self, offset: usize) -> Result<JavaValue> {
        profile::array_load_one_typed(Self::profile_array_type_index(&self.inner.element_type));

        Ok(match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let offset = offset * Self::primitive_element_size(&self.inner.element_type);
                Self::load_primitive_value(x, &self.inner.element_type, offset)
            }
            ArrayElements::NonPrimitive(x) => x.read()[offset].clone(),
        })
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
            match &self.inner.elements {
                ArrayElements::Primitive(values) => {
                    let element_size = Self::primitive_element_size(&self.inner.element_type);
                    let src_start = src_pos * element_size;
                    let src_end = src_start + length * element_size;
                    let dest_start = dest_pos * element_size;
                    let values_to_copy = load_bytes(&values[src_start..src_end]);
                    store_bytes(&values[dest_start..dest_start + values_to_copy.len()], &values_to_copy);
                }
                ArrayElements::NonPrimitive(values) => {
                    let mut values = values.write();
                    let values_to_copy = values[src_pos..src_pos + length].to_vec();
                    values.splice(dest_pos..dest_pos + length, values_to_copy);
                }
            }

            return Ok(true);
        }

        match (&self.inner.elements, &dest.inner.elements) {
            (ArrayElements::Primitive(src), ArrayElements::Primitive(dest)) => {
                let element_size = Self::primitive_element_size(&self.inner.element_type);
                let src_start = src_pos * element_size;
                let src_end = src_start + length * element_size;
                let dest_start = dest_pos * element_size;
                for (src, dest) in src[src_start..src_end]
                    .iter()
                    .zip(dest[dest_start..dest_start + length * element_size].iter())
                {
                    dest.store(src.load(Ordering::Relaxed), Ordering::Relaxed);
                }
            }
            (ArrayElements::NonPrimitive(src), ArrayElements::NonPrimitive(dest)) => {
                let values_to_copy = src.read()[src_pos..src_pos + length].to_vec();
                dest.write().splice(dest_pos..dest_pos + length, values_to_copy);
            }
            _ => return Ok(false),
        }

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

fn load_bytes(values: &[AtomicU8]) -> Vec<u8> {
    values.iter().map(|value| value.load(Ordering::Relaxed)).collect()
}

fn load_bytes_array<const N: usize>(values: &[AtomicU8]) -> [u8; N] {
    let mut result = [0; N];
    for (dest, value) in result.iter_mut().zip(values) {
        *dest = value.load(Ordering::Relaxed);
    }
    result
}

fn store_bytes(dest: &[AtomicU8], values: &[u8]) {
    for (dest, value) in dest.iter().zip(values) {
        dest.store(*value, Ordering::Relaxed);
    }
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

impl ArrayRawBuffer for ArrayRawBufferImpl {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        profile::array_raw_read();

        match &self.inner.elements {
            ArrayElements::Primitive(x) => {
                let element_size = ArrayClassInstanceImpl::primitive_element_size(&self.inner.element_type);
                let byte_offset = offset * element_size;
                let values_raw = &x[byte_offset..byte_offset + buffer.len()];

                for (dest, value) in buffer.iter_mut().zip(values_raw) {
                    *dest = value.load(Ordering::Relaxed);
                }
            }
            ArrayElements::NonPrimitive(_) => {
                panic!("Expected primitive array");
            }
        }

        Ok(())
    }
}

impl ArrayRawBufferMut for ArrayRawBufferImpl {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()> {
        profile::array_raw_write();

        if let ArrayElements::Primitive(x) = &self.inner.elements {
            let element_size = ArrayClassInstanceImpl::primitive_element_size(&self.inner.element_type);
            let byte_offset = offset * element_size;

            store_bytes(&x[byte_offset..byte_offset + buffer.len()], buffer);
        } else {
            panic!("Expected primitive array");
        }

        Ok(())
    }
}
