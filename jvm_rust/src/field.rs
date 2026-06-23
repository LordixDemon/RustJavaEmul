use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use core::{
    hash::{Hash, Hasher},
    sync::atomic::{AtomicUsize, Ordering},
};

use classfile::FieldInfo;
use java_class_proto::JavaFieldProto;
use java_constants::FieldAccessFlags;
use jvm::Field;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct FieldInner {
    slot: usize,
    name: String,
    descriptor: String,
    access_flags: FieldAccessFlags,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldImpl {
    inner: Arc<FieldInner>,
}

impl FieldImpl {
    pub fn new(name: &str, descriptor: &str, access_flags: FieldAccessFlags) -> Self {
        Self {
            inner: Arc::new(FieldInner {
                slot: next_field_slot(),
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                access_flags,
            }),
        }
    }

    pub fn from_field_proto(proto: JavaFieldProto) -> Self {
        Self::new(&proto.name, &proto.descriptor, proto.access_flags)
    }

    pub fn from_field_info(field_info: FieldInfo) -> Self {
        Self {
            inner: Arc::new(FieldInner {
                slot: next_field_slot(),
                name: field_info.name.to_string(),
                descriptor: field_info.descriptor.to_string(),
                access_flags: field_info.access_flags,
            }),
        }
    }

    pub fn descriptor_ref(&self) -> &str {
        &self.inner.descriptor
    }

    pub fn slot(&self) -> usize {
        self.inner.slot
    }
}

impl Hash for FieldImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.slot.hash(state);
        self.inner.name.hash(state);
        self.inner.descriptor.hash(state);
        self.inner.access_flags.bits().hash(state);
    }
}

fn next_field_slot() -> usize {
    static NEXT_FIELD_SLOT: AtomicUsize = AtomicUsize::new(0);
    NEXT_FIELD_SLOT.fetch_add(1, Ordering::Relaxed)
}

impl Field for FieldImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    fn access_flags(&self) -> FieldAccessFlags {
        self.inner.access_flags
    }
}
