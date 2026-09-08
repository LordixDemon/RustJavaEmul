#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{Runtime, RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
#[allow(unused_imports)]
use core::{
    sync::atomic::{AtomicI32, Ordering},
    time::Duration,
};
#[allow(unused_imports)]
use dyn_clone::clone_box;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
#[allow(unused_imports)]
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
#[allow(unused_imports)]
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

impl DiscoveryListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/DiscoveryListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract(
                    "deviceDiscovered",
                    "(Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DeviceClass;)V",
                    Default::default(),
                ),
                JavaMethodProto::new_abstract("inquiryCompleted", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("servicesDiscovered", "(I[Ljavax/bluetooth/ServiceRecord;)V", Default::default()),
                JavaMethodProto::new_abstract("serviceSearchCompleted", "(II)V", Default::default()),
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("INQUIRY_COMPLETED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INQUIRY_TERMINATED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INQUIRY_ERROR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SERVICE_SEARCH_COMPLETED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SERVICE_SEARCH_TERMINATED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SERVICE_SEARCH_ERROR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SERVICE_SEARCH_NO_RECORDS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "SERVICE_SEARCH_DEVICE_NOT_REACHABLE",
                    "I",
                    FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/bluetooth/DiscoveryListener";
        jvm.put_static_field(class, "INQUIRY_COMPLETED", "I", 0x00).await?;
        jvm.put_static_field(class, "INQUIRY_TERMINATED", "I", 0x05).await?;
        jvm.put_static_field(class, "INQUIRY_ERROR", "I", 0x07).await?;
        jvm.put_static_field(class, "SERVICE_SEARCH_COMPLETED", "I", 0x01).await?;
        jvm.put_static_field(class, "SERVICE_SEARCH_TERMINATED", "I", 0x02).await?;
        jvm.put_static_field(class, "SERVICE_SEARCH_ERROR", "I", 0x03).await?;
        jvm.put_static_field(class, "SERVICE_SEARCH_NO_RECORDS", "I", 0x04).await?;
        jvm.put_static_field(class, "SERVICE_SEARCH_DEVICE_NOT_REACHABLE", "I", 0x06).await
    }
}
