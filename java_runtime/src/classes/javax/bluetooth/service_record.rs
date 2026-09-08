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

impl ServiceRecord {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/ServiceRecord",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getAttributeValue",
                    "(I)Ljavax/bluetooth/DataElement;",
                    Self::get_attribute_value,
                    Default::default(),
                ),
                JavaMethodProto::new("getConnectionURL", "(IZ)Ljava/lang/String;", Self::get_connection_url, Default::default()),
                JavaMethodProto::new(
                    "getHostDevice",
                    "()Ljavax/bluetooth/RemoteDevice;",
                    Self::get_host_device,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setAttributeValue",
                    "(ILjavax/bluetooth/DataElement;)Z",
                    Self::set_attribute_value,
                    Default::default(),
                ),
                JavaMethodProto::new("setDeviceServiceClasses", "(I)V", Self::set_device_service_classes, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("host", "Ljavax/bluetooth/RemoteDevice;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn get_attribute_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _id: i32,
    ) -> Result<ClassInstanceRef<DataElement>> {
        Ok(jvm.new_class("javax/bluetooth/DataElement", "()V", ()).await?.into())
    }

    pub(super) async fn get_connection_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _security: i32,
        _master: bool,
    ) -> Result<ClassInstanceRef<String>> {
        let host: ClassInstanceRef<RemoteDevice> = jvm.get_field(&this, "host", "Ljavax/bluetooth/RemoteDevice;").await?;
        let address = if host.is_null() {
            alloc::string::String::from("000000000001")
        } else {
            let addr: ClassInstanceRef<String> = jvm.invoke_virtual(&host, "getBluetoothAddress", "()Ljava/lang/String;", ()).await?;
            if addr.is_null() {
                alloc::string::String::from("000000000001")
            } else {
                JavaLangString::to_rust_string(jvm, &addr).await?
            }
        };
        JavaLangString::from_rust_string(jvm, &format!("btgoep://{address}:9"))
            .await
            .map(Into::into)
    }

    pub(super) async fn get_host_device(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<RemoteDevice>> {
        jvm.get_field(&this, "host", "Ljavax/bluetooth/RemoteDevice;").await
    }

    pub(super) async fn set_attribute_value(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _id: i32,
        _value: ClassInstanceRef<DataElement>,
    ) -> Result<bool> {
        Ok(true)
    }

    stub_void! {
        set_device_service_classes(_classes: i32);
    }
}
