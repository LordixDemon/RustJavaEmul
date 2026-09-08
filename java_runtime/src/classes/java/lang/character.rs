use alloc::{string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Character
pub struct Character;

impl Character {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Character",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(C)V", Self::init, Default::default()),
                JavaMethodProto::new("charValue", "()C", Self::char_value, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("digit", "(CI)I", Self::digit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isDigit", "(C)Z", Self::is_digit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isLowerCase", "(C)Z", Self::is_lower_case, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isUpperCase", "(C)Z", Self::is_upper_case, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toLowerCase", "(C)C", Self::to_lower_case, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toUpperCase", "(C)C", Self::to_upper_case, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(C)Ljava/lang/Character;", Self::value_of, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "C", Default::default()),
                JavaFieldProto::new("MIN_VALUE", "C", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_VALUE", "C", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MIN_RADIX", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAX_RADIX", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE", "Ljava/lang/Class;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Character", "MIN_VALUE", "C", 0 as JavaChar).await?;
        jvm.put_static_field("java/lang/Character", "MAX_VALUE", "C", 0xffff as JavaChar).await?;
        jvm.put_static_field("java/lang/Character", "MIN_RADIX", "I", 2).await?;
        jvm.put_static_field("java/lang/Character", "MAX_RADIX", "I", 36).await?;
        let class = jvm.resolve_class("java/lang/Character").await?;
        jvm.put_static_field("java/lang/Character", "TYPE", "Ljava/lang/Class;", class.java_class())
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "C", value).await
    }

    async fn char_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "value", "C").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Character" {
            return Ok(false);
        }
        Ok(jvm.get_field::<JavaChar>(&this, "value", "C").await? == jvm.get_field::<JavaChar>(&other, "value", "C").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<JavaChar>(&this, "value", "C").await? as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: JavaChar = jvm.get_field(&this, "value", "C").await?;
        let ch = char::from_u32(value as u32).unwrap_or('\u{FFFD}');
        Ok(JavaLangString::from_rust_string(jvm, &ch.to_string()).await?.into())
    }

    async fn digit(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar, radix: i32) -> Result<i32> {
        if !(2..=36).contains(&radix) {
            return Ok(-1);
        }
        let digit = match ch {
            ch if (b'0' as JavaChar..=b'9' as JavaChar).contains(&ch) => (ch - b'0' as JavaChar) as i32,
            ch if (b'a' as JavaChar..=b'z' as JavaChar).contains(&ch) => (ch - b'a' as JavaChar) as i32 + 10,
            ch if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&ch) => (ch - b'A' as JavaChar) as i32 + 10,
            _ => return Ok(-1),
        };
        Ok(if digit < radix { digit } else { -1 })
    }

    async fn is_digit(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar) -> Result<bool> {
        Ok((b'0' as JavaChar..=b'9' as JavaChar).contains(&ch))
    }

    async fn is_lower_case(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar) -> Result<bool> {
        Ok((b'a' as JavaChar..=b'z' as JavaChar).contains(&ch))
    }

    async fn is_upper_case(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar) -> Result<bool> {
        Ok((b'A' as JavaChar..=b'Z' as JavaChar).contains(&ch))
    }

    async fn to_lower_case(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar) -> Result<JavaChar> {
        if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&ch) {
            Ok(ch + 32)
        } else {
            Ok(ch)
        }
    }

    async fn to_upper_case(_: &Jvm, _: &mut RuntimeContext, ch: JavaChar) -> Result<JavaChar> {
        if (b'a' as JavaChar..=b'z' as JavaChar).contains(&ch) {
            Ok(ch - 32)
        } else {
            Ok(ch)
        }
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Character", "(C)V", (value,)).await?.into())
    }
}
