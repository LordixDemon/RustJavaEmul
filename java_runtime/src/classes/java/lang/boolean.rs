use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Boolean
pub struct Boolean;

impl Boolean {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Boolean",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Z)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, Default::default()),
                JavaMethodProto::new("booleanValue", "()Z", Self::boolean_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(Z)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(Z)Ljava/lang/Boolean;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Boolean;",
                    Self::value_of_string,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "Z", Default::default()),
                JavaFieldProto::new("TRUE", "Ljava/lang/Boolean;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FALSE", "Ljava/lang/Boolean;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let true_value = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?;
        let false_value = jvm.new_class("java/lang/Boolean", "(Z)V", (false,)).await?;
        jvm.put_static_field("java/lang/Boolean", "TRUE", "Ljava/lang/Boolean;", true_value)
            .await?;
        jvm.put_static_field("java/lang/Boolean", "FALSE", "Ljava/lang/Boolean;", false_value)
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "Z", value).await
    }

    async fn init_string(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        let value = Self::parse_boolean(jvm, &s).await?;
        Self::init(jvm, context, this, value).await
    }

    async fn boolean_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "value", "Z").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if other.class_definition().name() != "java/lang/Boolean" {
            return Ok(false);
        }
        let a: bool = jvm.get_field(&this, "value", "Z").await?;
        let b: bool = jvm.get_field(&other, "value", "Z").await?;
        Ok(a == b)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: bool = jvm.get_field(&this, "value", "Z").await?;
        Ok(if value { 1231 } else { 1237 })
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: bool = jvm.get_field(&this, "value", "Z").await?;
        Ok(JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?.into())
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<Self>> {
        let field = if value { "TRUE" } else { "FALSE" };
        jvm.get_static_field("java/lang/Boolean", field, "Ljava/lang/Boolean;").await
    }

    async fn value_of_string(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value = Self::parse_boolean(jvm, &s).await?;
        Self::value_of(jvm, context, value).await
    }

    async fn parse_boolean(jvm: &Jvm, s: &ClassInstanceRef<String>) -> Result<bool> {
        if s.is_null() {
            return Ok(false);
        }
        let rust = JavaLangString::to_rust_string(jvm, s).await?;
        Ok(rust.eq_ignore_ascii_case("true"))
    }
}
