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

impl UUID {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/bluetooth/UUID",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(J)V", Self::init_long, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Z)V", Self::init_string, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("value", "Ljava/lang/String;", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let text = JavaLangString::from_rust_string(jvm, &format!("{value:x}")).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/String;", text).await
    }

    pub(super) async fn init_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<String>,
        _short_uuid: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/String;", value).await
    }

    pub(super) async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "value", "Ljava/lang/String;").await
    }

    pub(super) async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        let a: ClassInstanceRef<String> = jvm.get_field(&this, "value", "Ljava/lang/String;").await?;
        let b: ClassInstanceRef<String> = jvm.get_field(&other, "value", "Ljava/lang/String;").await?;
        if a.is_null() || b.is_null() {
            return Ok(a.is_null() && b.is_null());
        }
        let left = JavaLangString::to_rust_string(jvm, &a).await?;
        let right = JavaLangString::to_rust_string(jvm, &b).await?;
        Ok(left.eq_ignore_ascii_case(&right))
    }
}
