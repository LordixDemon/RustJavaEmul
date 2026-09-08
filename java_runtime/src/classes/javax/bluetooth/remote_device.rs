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

impl RemoteDevice {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/RemoteDevice",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_addr, Default::default()),
                JavaMethodProto::new("getBluetoothAddress", "()Ljava/lang/String;", Self::get_address, Default::default()),
                JavaMethodProto::new("getFriendlyName", "(Z)Ljava/lang/String;", Self::get_friendly_name, Default::default()),
                JavaMethodProto::new(
                    "getRemoteDevice",
                    "(Ljavax/microedition/io/Connection;)Ljavax/bluetooth/RemoteDevice;",
                    Self::get_remote_device,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("address", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("name", "Ljava/lang/String;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn init_addr(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        address: ClassInstanceRef<String>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "address", "Ljava/lang/String;", address).await
    }

    pub(super) async fn get_address(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "address", "Ljava/lang/String;").await
    }

    pub(super) async fn get_friendly_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _always: bool,
    ) -> Result<ClassInstanceRef<String>> {
        let name: ClassInstanceRef<String> = jvm.get_field(&this, "name", "Ljava/lang/String;").await?;
        if name.is_null() {
            JavaLangString::from_rust_string(jvm, "unknown").await.map(Into::into)
        } else {
            Ok(name)
        }
    }

    pub(super) async fn get_remote_device(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _conn: ClassInstanceRef<crate::classes::java::lang::Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("javax/bluetooth/RemoteDevice", "()V", ()).await?.into())
    }
}
