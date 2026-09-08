use alloc::{boxed::Box, vec::Vec};

use crate::{Result, class_definition::ClassDefinition, class_instance::ClassInstance, field::Field, value::JavaValue};

#[async_trait::async_trait]
pub trait ArrayClassInstance: ClassInstance {
    fn class_definition(&self) -> Box<dyn ClassDefinition>;
    fn destroy(self: Box<Self>);
    fn equals(&self, other: &dyn ClassInstance) -> Result<bool>;
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()>;
    fn load(&self, offset: usize, count: usize) -> Result<Vec<JavaValue>>;
    fn store_one(&mut self, offset: usize, value: JavaValue) -> Result<()> {
        self.store(offset, Box::new([value]))
    }
    fn load_one(&self, offset: usize) -> Result<JavaValue> {
        Ok(self.load(offset, 1)?.remove(0))
    }
    fn copy_to(&self, _src_pos: usize, _dest: &mut dyn ArrayClassInstance, _dest_pos: usize, _length: usize) -> Result<bool> {
        Ok(false)
    }
    fn raw_buffer(&self) -> Result<Box<dyn ArrayRawBuffer>>;
    fn raw_buffer_mut(&mut self) -> Result<Box<dyn ArrayRawBufferMut>>;
    fn length(&self) -> usize;
    fn is_object_array(&self) -> bool {
        false
    }
    fn i32_slice(&self) -> Option<&[i32]> {
        None
    }
    fn i32_slice_mut(&mut self) -> Option<&mut [i32]> {
        None
    }
}

#[async_trait::async_trait]
impl<T: ArrayClassInstance> ClassInstance for T {
    fn destroy(self: Box<Self>) {
        ArrayClassInstance::destroy(self)
    }

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        ArrayClassInstance::class_definition(self)
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        ArrayClassInstance::equals(self, other)
    }
    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn Field) -> Result<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn Field, _value: JavaValue) -> Result<()> {
        panic!("Array classes do not have fields")
    }
}

pub trait ArrayRawBuffer: Send {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()>;
    fn i32_slice(&self) -> Option<&[i32]> {
        None
    }
}

pub trait ArrayRawBufferMut: ArrayRawBuffer {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()>;
    fn i32_slice_mut(&mut self) -> Option<&mut [i32]> {
        None
    }
    fn fill_i32(&mut self, offset: usize, count: usize, value: i32) -> Result<()> {
        if let Some(slice) = self.i32_slice_mut() {
            let end = offset.saturating_add(count);
            if end <= slice.len() {
                slice[offset..end].fill(value);
                return Ok(());
            }
        }
        if count == 0 {
            return Ok(());
        }
        let bytes = value.to_ne_bytes();
        for index in 0..count {
            self.write(offset + index, &bytes)?;
        }
        Ok(())
    }
}
