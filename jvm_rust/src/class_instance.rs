use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    any::Any,
    cell::UnsafeCell,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU32, Ordering},
};

use parking_lot::RwLock;

use jvm::{ClassDefinition, ClassInstance, Field, JavaType, JavaValue, Result};

use crate::{FieldImpl, class_definition::ClassDefinitionImpl};

struct ClassInstanceInner {
    class: Box<dyn ClassDefinition>,
    storage: RwLock<Vec<Option<JavaValue>>>,
    native: UnsafeCell<Option<Box<dyn Any + Send + Sync>>>,
    native_epoch: AtomicU32,
}

unsafe impl Sync for ClassInstanceInner {}

#[derive(Clone)]
pub struct ClassInstanceImpl {
    inner: Arc<ClassInstanceInner>,
}

impl ClassInstanceImpl {
    pub fn store_i32_slot(&self, slot: usize, value: i32) {
        let mut storage = self.inner.storage.write();
        if storage.len() <= slot {
            storage.resize_with(slot + 1, || None);
        }
        storage[slot] = Some(JavaValue::Int(value));
    }

    pub fn store_named_i32(&self, name: &str, value: i32) -> bool {
        let Some(class) = self.inner.class.as_any().downcast_ref::<ClassDefinitionImpl>() else {
            return false;
        };
        let Some(field) = class.field_impl(name, "I", false) else {
            return false;
        };
        self.store_i32_slot(field.slot(), value);
        true
    }

    pub fn from_instance(instance: &dyn ClassInstance) -> Option<Self> {
        instance.as_any().downcast_ref::<Self>().cloned()
    }

    pub fn named_i32(&self, name: &str) -> i32 {
        let Some(class) = self.inner.class.as_any().downcast_ref::<ClassDefinitionImpl>() else {
            return 0;
        };
        let Some(field) = class.field_impl(name, "I", false) else {
            return 0;
        };
        let storage = self.inner.storage.read();
        match storage.get(field.slot()) {
            Some(Some(JavaValue::Int(value))) => *value,
            Some(Some(JavaValue::Boolean(value))) => i32::from(*value),
            Some(Some(JavaValue::Byte(value))) => i32::from(*value),
            Some(Some(JavaValue::Short(value))) => i32::from(*value),
            Some(Some(JavaValue::Char(value))) => i32::from(*value),
            _ => 0,
        }
    }

    pub fn named_instance(&self, name: &str, descriptor: &str) -> Option<Self> {
        let class = self.inner.class.as_any().downcast_ref::<ClassDefinitionImpl>()?;
        let field = class.field_impl(name, descriptor, false)?;
        let storage = self.inner.storage.read();
        match storage.get(field.slot()) {
            Some(Some(JavaValue::Object(Some(obj)))) => Self::from_instance(obj.as_ref()),
            _ => None,
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn native_mut<T: Any + 'static>(&self) -> Option<&mut T> {
        unsafe { (*self.inner.native.get()).as_mut()?.downcast_mut() }
    }

    pub fn new(class: &ClassDefinitionImpl) -> Self {
        Self {
            inner: Arc::new(ClassInstanceInner {
                class: Box::new(class.clone()),
                storage: RwLock::new(Vec::new()),
                native: UnsafeCell::new(None),
                native_epoch: AtomicU32::new(0),
            }),
        }
    }
}

#[async_trait::async_trait]
impl ClassInstance for ClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.inner.class.clone()
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ClassInstanceImpl>();
        if other.is_none() {
            return Ok(false);
        }
        let other = other.unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn get_field(&self, field: &dyn Field) -> Result<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();
        let slot = field.slot();

        let storage = self.inner.storage.read();

        if let Some(Some(x)) = storage.get(slot) {
            Ok(x.clone())
        } else {
            Ok(JavaType::parse(field.descriptor_ref()).default())
        }
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();
        let slot = field.slot();

        let mut storage = self.inner.storage.write();
        if storage.len() <= slot {
            storage.resize_with(slot + 1, || None);
        }
        storage[slot] = Some(value);

        Ok(())
    }

    fn get_named_field(&self, name: &str, descriptor: &str) -> Option<JavaValue> {
        let class = self.inner.class.as_any().downcast_ref::<ClassDefinitionImpl>()?;
        let field = class.field_impl(name, descriptor, false)?;
        self.get_field(field).ok()
    }

    fn put_named_field(&mut self, name: &str, descriptor: &str, value: JavaValue) -> bool {
        let Some(class) = self.inner.class.as_any().downcast_ref::<ClassDefinitionImpl>() else {
            return false;
        };
        let Some(field) = class.field_impl(name, descriptor, false).cloned() else {
            return false;
        };
        self.put_field(&field, value).is_ok()
    }

    fn with_native_scratch(&self, f: &mut dyn FnMut(&mut Option<Box<dyn Any + Send + Sync>>)) {
        f(unsafe { &mut *self.inner.native.get() })
    }

    fn native_epoch(&self) -> u32 {
        self.inner.native_epoch.load(Ordering::Relaxed)
    }

    fn clear_native_scratch(&self) {
        self.inner.native_epoch.fetch_add(1, Ordering::Relaxed);
        self.with_native_scratch(&mut |slot| *slot = None);
    }
}

impl Hash for ClassInstanceImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl Debug for ClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ClassInstance({})", self.inner.class.name())
    }
}
