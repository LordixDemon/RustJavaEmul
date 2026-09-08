#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::{boxed::Box, format, vec};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
#[allow(unused_imports)]
use java_constants::{FieldAccessFlags, MethodAccessFlags};
#[allow(unused_imports)]
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl DeviceClass {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/DeviceClass",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_value, Default::default()),
                JavaMethodProto::new("getMajorDeviceClass", "()I", Self::get_major, Default::default()),
                JavaMethodProto::new("getMinorDeviceClass", "()I", Self::get_minor, Default::default()),
                JavaMethodProto::new("getServiceClasses", "()I", Self::get_service, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("cod", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::init_value(jvm, context, this, 0).await
    }

    pub(super) async fn init_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, cod: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "cod", "I", cod).await
    }

    pub(super) async fn get_major(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let cod: i32 = jvm.get_field(&this, "cod", "I").await?;
        Ok(cod & 0x1f00)
    }

    pub(super) async fn get_minor(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let cod: i32 = jvm.get_field(&this, "cod", "I").await?;
        Ok(cod & 0xfc)
    }

    pub(super) async fn get_service(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let cod: i32 = jvm.get_field(&this, "cod", "I").await?;
        Ok(cod & 0xffe000)
    }
}
