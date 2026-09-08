use alloc::boxed::Box;
use core::{
    any::Any,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use dyn_clone::{DynClone, clone_trait_object};
use dyn_hash::{DynHash, hash_trait_object};

use crate::{ArrayClassInstance, ClassDefinition, Field, Result, as_any::AsAny, value::JavaValue};

#[async_trait::async_trait]
pub trait ClassInstance: Sync + Send + AsAny + Debug + DynHash + DynClone + 'static {
    fn destroy(self: Box<Self>);
    fn class_definition(&self) -> Box<dyn ClassDefinition>;
    fn equals(&self, other: &dyn ClassInstance) -> Result<bool>;
    fn get_field(&self, field: &dyn Field) -> Result<JavaValue>;
    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()>;
    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        None
    }
    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
    fn get_named_field(&self, name: &str, descriptor: &str) -> Option<JavaValue> {
        let field = self.class_definition().field(name, descriptor, false)?;
        self.get_field(&*field).ok()
    }
    fn put_named_field(&mut self, name: &str, descriptor: &str, value: JavaValue) -> bool {
        let Some(field) = self.class_definition().field(name, descriptor, false) else {
            return false;
        };
        self.put_field(&*field, value).is_ok()
    }
    fn with_native_scratch(&self, _f: &mut dyn FnMut(&mut Option<Box<dyn Any + Send + Sync>>)) {}
    fn native_epoch(&self) -> u32 {
        0
    }
    fn clear_native_scratch(&self) {
        self.with_native_scratch(&mut |slot| *slot = None);
    }
}

clone_trait_object!(ClassInstance);
hash_trait_object!(ClassInstance);

impl Eq for dyn ClassInstance {}
impl PartialEq for dyn ClassInstance {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other).unwrap()
    }
}

// array wrapper for ClassInstanceRef
pub struct Array<T>(PhantomData<T>);

// typesafe wrapper for ClassInstance
pub struct ClassInstanceRef<T> {
    pub instance: Option<Box<dyn ClassInstance>>,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> ClassInstanceRef<T> {
    pub fn new(instance: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            instance,
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for ClassInstanceRef<T> {
    fn clone(&self) -> Self {
        Self {
            instance: self.instance.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> ClassInstanceRef<T> {
    pub fn is_null(&self) -> bool {
        self.instance
            .as_ref()
            .is_none_or(|instance| instance.as_any().downcast_ref::<NullInstance>().is_some())
    }
}

impl<T> Debug for ClassInstanceRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(x) = &self.instance {
            write!(f, "{x:?}")
        } else {
            write!(f, "null")
        }
    }
}

impl<T> Deref for ClassInstanceRef<T> {
    type Target = Box<dyn ClassInstance>;
    fn deref(&self) -> &Self::Target {
        match self.instance.as_ref() {
            Some(instance) => instance,
            None => java_null_box(),
        }
    }
}

impl<T> DerefMut for ClassInstanceRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.instance.is_none() {
            self.instance = Some(Box::new(NullInstance));
        }
        self.instance.as_mut().unwrap()
    }
}

impl<T> From<ClassInstanceRef<T>> for JavaValue {
    fn from(value: ClassInstanceRef<T>) -> Self {
        let instance: Option<Box<dyn ClassInstance>> = value.into();
        instance.into()
    }
}

impl<T> From<Box<dyn ClassInstance>> for ClassInstanceRef<T> {
    fn from(value: Box<dyn ClassInstance>) -> Self {
        Self {
            instance: Some(value),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<Option<Box<dyn ClassInstance>>> for ClassInstanceRef<T> {
    fn from(value: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            instance: value,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<JavaValue> for ClassInstanceRef<T> {
    fn from(val: JavaValue) -> Self {
        ClassInstanceRef {
            instance: val.into(),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<ClassInstanceRef<T>> for Box<dyn ClassInstance> {
    fn from(value: ClassInstanceRef<T>) -> Self {
        value.instance.unwrap_or_else(|| Box::new(NullInstance))
    }
}

impl<T> From<ClassInstanceRef<T>> for Option<Box<dyn ClassInstance>> {
    fn from(value: ClassInstanceRef<T>) -> Self {
        match value.instance {
            Some(instance) if instance.as_any().downcast_ref::<NullInstance>().is_some() => None,
            instance => instance,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NullInstance;

impl Hash for NullInstance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        0u8.hash(state);
    }
}

#[async_trait::async_trait]
impl ClassInstance for NullInstance {
    fn destroy(self: Box<Self>) {}

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        panic!("java null");
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        Ok(other.as_any().downcast_ref::<NullInstance>().is_some())
    }

    fn get_field(&self, _: &dyn Field) -> Result<JavaValue> {
        panic!("java null");
    }

    fn put_field(&mut self, _: &dyn Field, _: JavaValue) -> Result<()> {
        panic!("java null");
    }
}

#[allow(clippy::borrowed_box)]
fn java_null_box() -> &'static Box<dyn ClassInstance> {
    use parking_lot::Mutex;
    static SLOT: Mutex<Option<&'static Box<dyn ClassInstance>>> = Mutex::new(None);
    let mut slot = SLOT.lock();
    if slot.is_none() {
        let boxed: Box<dyn ClassInstance> = Box::new(NullInstance);
        *slot = Some(Box::leak(Box::new(boxed)));
    }
    slot.as_ref().copied().unwrap()
}
