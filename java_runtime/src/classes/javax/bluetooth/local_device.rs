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

impl LocalDevice {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/LocalDevice",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getDiscoveryAgent",
                    "()Ljavax/bluetooth/DiscoveryAgent;",
                    Self::get_discovery_agent,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getLocalDevice",
                    "()Ljavax/bluetooth/LocalDevice;",
                    Self::get_local_device,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("setDiscoverable", "(I)Z", Self::set_discoverable, Default::default()),
                JavaMethodProto::new("getDiscoverable", "()I", Self::get_discoverable, Default::default()),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getRecord",
                    "(Ljavax/microedition/io/Connection;)Ljavax/bluetooth/ServiceRecord;",
                    Self::get_record,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "updateRecord",
                    "(Ljavax/bluetooth/ServiceRecord;)V",
                    Self::update_record,
                    Default::default(),
                ),
                JavaMethodProto::new("getBluetoothAddress", "()Ljava/lang/String;", Self::get_address, Default::default()),
                JavaMethodProto::new("getFriendlyName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new(
                    "getDeviceClass",
                    "()Ljavax/bluetooth/DeviceClass;",
                    Self::get_device_class,
                    Default::default(),
                ),
                JavaMethodProto::new("isPowerOn", "()Z", Self::is_power_on, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("instance", "Ljavax/bluetooth/LocalDevice;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("agent", "Ljavax/bluetooth/DiscoveryAgent;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let agent = jvm.new_class("javax/bluetooth/DiscoveryAgent", "()V", ()).await?;
        jvm.put_field(&mut this, "agent", "Ljavax/bluetooth/DiscoveryAgent;", agent).await
    }

    pub(super) async fn get_discovery_agent(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<DiscoveryAgent>> {
        jvm.get_field(&this, "agent", "Ljavax/bluetooth/DiscoveryAgent;").await
    }

    pub(super) async fn get_local_device(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let existing: ClassInstanceRef<Self> = jvm
            .get_static_field("javax/bluetooth/LocalDevice", "instance", "Ljavax/bluetooth/LocalDevice;")
            .await?;
        if !existing.is_null() {
            return Ok(existing);
        }
        let created: ClassInstanceRef<Self> = jvm.new_class("javax/bluetooth/LocalDevice", "()V", ()).await?.into();
        jvm.put_static_field(
            "javax/bluetooth/LocalDevice",
            "instance",
            "Ljavax/bluetooth/LocalDevice;",
            created.clone(),
        )
        .await?;
        Ok(created)
    }

    pub(super) async fn set_discoverable(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _mode: i32) -> Result<bool> {
        Ok(true)
    }

    pub(super) async fn get_discoverable(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    pub(super) async fn get_property(jvm: &Jvm, _: &mut RuntimeContext, _name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }

    pub(super) async fn get_record(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _connection: ClassInstanceRef<crate::classes::java::lang::Object>,
    ) -> Result<ClassInstanceRef<ServiceRecord>> {
        Ok(jvm.new_class("javax/bluetooth/ServiceRecord", "()V", ()).await?.into())
    }

    stub_void! {
        update_record(_record: ClassInstanceRef<ServiceRecord>);
    }

    pub(super) async fn get_address(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "000000000000").await.map(Into::into)
    }

    pub(super) async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "RustJava").await.map(Into::into)
    }

    pub(super) async fn get_device_class(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<DeviceClass>> {
        Ok(jvm.new_class("javax/bluetooth/DeviceClass", "()V", ()).await?.into())
    }

    pub(super) async fn is_power_on(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }
}
