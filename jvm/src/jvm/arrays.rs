use alloc::{boxed::Box, format, vec::Vec};
use core::iter;

use crate::{
    Result,
    array_class_instance::{ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut},
    class_instance::ClassInstance,
    r#type::JavaType,
    value::JavaValue,
};

use super::Jvm;

impl Jvm {
    pub async fn instantiate_array(&self, element_type_name: &str, length: usize) -> Result<Box<dyn ClassInstance>> {
        tracing::trace!("Instantiate array of {} with length {}", element_type_name, length);

        let class_name = format!("[{element_type_name}");

        let class = self.resolve_class(&class_name).await?.definition;
        let array_class = class.as_array_class_definition().unwrap();

        let instance = array_class.instantiate_array(self, length).await?;

        let thread_id = (self.inner.get_current_thread_id)();
        let mut threads = self.inner.threads.write();
        let thread = threads.get_mut(&thread_id).unwrap();

        thread.top_frame_mut().local_variables_mut().push(instance.clone());
        self.inner.all_objects.write().insert(instance.clone());

        Ok(instance)
    }
    #[async_recursion::async_recursion]
    pub async fn store_array<T, U>(&self, array: &mut Box<dyn ClassInstance>, offset: usize, values: T) -> Result<()>
    where
        T: IntoIterator<Item = U> + Send,
        U: Into<JavaValue>,
    {
        let array = self.ensure_not_null_mut(array).await?;
        tracing::trace!("Store array {} at offset {}", array.class_definition().name(), offset);

        let values = values.into_iter().map(|x| x.into()).collect::<Vec<_>>();

        let array_size = self.array_length(array).await?;
        if offset + values.len() > array_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", offset + values.len(), array_size),
                )
                .await);
        }

        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.store(offset, values.into_boxed_slice())?;

            Ok(())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn load_array<T>(&self, array: &Box<dyn ClassInstance>, offset: usize, count: usize) -> Result<Vec<T>>
    where
        T: From<JavaValue>,
    {
        let array = self.ensure_not_null(array).await?;
        tracing::trace!("Load array {} at offset {}", array.class_definition().name(), offset);

        let array_size = self.array_length(array).await?;
        if offset + count > array_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", offset + count, array_size),
                )
                .await);
        }

        let array = array.as_array_instance();

        if let Some(array) = array {
            let values = array.load(offset, count)?;

            Ok(iter::IntoIterator::into_iter(values).map(|x| x.into()).collect::<Vec<_>>())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn copy_array(
        &self,
        src: &Box<dyn ClassInstance>,
        src_pos: usize,
        dest: &mut Box<dyn ClassInstance>,
        dest_pos: usize,
        length: usize,
    ) -> Result<()> {
        let src = self.ensure_not_null(src).await?;
        let dest = self.ensure_not_null_mut(dest).await?;
        tracing::trace!(
            "Copy array {} at offset {} to {} at offset {} length {}",
            src.class_definition().name(),
            src_pos,
            dest.class_definition().name(),
            dest_pos,
            length
        );

        let src_size = self.array_length(src).await?;
        let dest_size = self.array_length(dest).await?;
        if src_pos + length > src_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", src_pos + length, src_size),
                )
                .await);
        }
        if dest_pos + length > dest_size {
            return Err(self
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &format!("{} > {}", dest_pos + length, dest_size),
                )
                .await);
        }

        let src_array = src.as_array_instance();
        let dest_array = dest.as_array_instance_mut();

        if let (Some(src_array), Some(dest_array)) = (src_array, dest_array) {
            if src_array.copy_to(src_pos, dest_array, dest_pos, length)? {
                return Ok(());
            }

            let values = src_array.load(src_pos, length)?;
            dest_array.store(dest_pos, values.into_boxed_slice())?;

            Ok(())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_raw_buffer(&self, array: &Box<dyn ClassInstance>) -> Result<Box<dyn ArrayRawBuffer>> {
        let array = self.ensure_not_null(array).await?;
        let array = array.as_array_instance();

        if let Some(array) = array {
            array.raw_buffer()
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_raw_buffer_mut(&self, array: &mut Box<dyn ClassInstance>) -> Result<Box<dyn ArrayRawBufferMut>> {
        let array = self.ensure_not_null_mut(array).await?;
        let array = array.as_array_instance_mut();

        if let Some(array) = array {
            array.raw_buffer_mut()
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_length(&self, array: &Box<dyn ClassInstance>) -> Result<usize> {
        let array = self.ensure_not_null(array).await?;
        tracing::trace!("Get array length {}", array.class_definition().name());

        let array = array.as_array_instance();

        if let Some(array) = array {
            Ok(array.length())
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }

    pub async fn array_element_type(&self, array: &Box<dyn ClassInstance>) -> Result<JavaType> {
        let array = self.ensure_not_null(array).await?;
        tracing::trace!("Get array element type {}", array.class_definition().name());

        let array = array.as_array_instance();

        if let Some(array) = array {
            let class = ArrayClassInstance::class_definition(array);

            let type_name = &class.name()[1..]; // TODO can we store JavaType on class?

            Ok(JavaType::parse(type_name))
        } else {
            Err(self.exception("java/lang/IllegalArgumentException", "Not an array").await)
        }
    }
}
