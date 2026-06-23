use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.Integer
pub struct Integer;

impl Integer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Integer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, Default::default()),
                JavaMethodProto::new("parseInt", "(Ljava/lang/String;)I", Self::parse_int, MethodAccessFlags::STATIC),
                JavaMethodProto::new("parseInt", "(Ljava/lang/String;I)I", Self::parse_int_radix, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/Integer;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Integer;",
                    Self::value_of_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(I)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toHexString", "(I)Ljava/lang/String;", Self::to_hex_string, MethodAccessFlags::STATIC),
            ],
            fields: vec![JavaFieldProto::new("value", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.lang.Integer::<init>({:?}, {:?})", &this, value);

        jvm.put_field(&mut this, "value", "I", value).await?;

        Ok(())
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Integer::intValue({:?})", &this);

        let value = jvm.get_field(&this, "value", "I").await?;

        Ok(value)
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Integer::valueOf({:?})", value);

        let instance = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?;

        Ok(instance.into())
    }

    async fn value_of_string(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Integer::valueOf({:?})", &s);

        let value = Self::parse_int(jvm, context, s).await?;
        Self::value_of(jvm, context, value).await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toString({:?})", &this);

        let value: i32 = jvm.get_field(&this, "value", "I").await?;

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn parse_int(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?})", &s);

        if s.is_null() {
            return Self::number_format_exception(jvm, "null").await;
        }

        let chars = jvm.get_field(&s, "value", "[C").await?;
        let length = jvm.array_length(&chars).await?;
        if length == 0 || length > 64 {
            return Self::number_format_exception(jvm, "").await;
        }

        let chars: alloc::vec::Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        match Self::parse_decimal_chars(&chars) {
            Some(value) => Ok(value),
            None => {
                let s = alloc::string::String::from_utf16_lossy(&chars);
                Self::number_format_exception(jvm, &s).await
            }
        }
    }

    async fn parse_int_radix(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>, radix: i32) -> Result<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?}, {:?})", &s, radix);

        if s.is_null() {
            return Self::number_format_exception(jvm, "null").await;
        }
        if !(2..=36).contains(&radix) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "radix out of range").await);
        }

        let chars = jvm.get_field(&s, "value", "[C").await?;
        let length = jvm.array_length(&chars).await?;
        if length == 0 || length > 64 {
            return Self::number_format_exception(jvm, "").await;
        }

        let chars: alloc::vec::Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        match Self::parse_chars_radix(&chars, radix) {
            Some(value) => Ok(value),
            None => {
                let s = alloc::string::String::from_utf16_lossy(&chars);
                Self::number_format_exception(jvm, &s).await
            }
        }
    }

    fn parse_decimal_chars(chars: &[JavaChar]) -> Option<i32> {
        Self::parse_chars_radix(chars, 10)
    }

    fn parse_chars_radix(chars: &[JavaChar], radix: i32) -> Option<i32> {
        let mut index = 0;
        let mut negative = false;
        match chars.first().copied() {
            Some(ch) if ch == b'-' as JavaChar => {
                negative = true;
                index = 1;
            }
            Some(ch) if ch == b'+' as JavaChar => {
                index = 1;
            }
            _ => {}
        }

        if index == chars.len() {
            return None;
        }

        let mut value = 0i64;
        while index < chars.len() {
            let ch = chars[index];
            let digit = char_digit(ch)? as i64;
            if digit >= radix as i64 {
                return None;
            }
            value = value.checked_mul(radix as i64)?.checked_add(digit)?;
            let limit = if negative { -(i32::MIN as i64) } else { i32::MAX as i64 };
            if value > limit {
                return None;
            }

            index += 1;
        }

        Some(if negative {
            if value == -(i32::MIN as i64) { i32::MIN } else { -(value as i32) }
        } else {
            value as i32
        })
    }

    async fn number_format_exception<T>(jvm: &Jvm, s: &str) -> Result<T> {
        Err(jvm
            .exception("java/lang/NumberFormatException", &format!("For input string: \"{s}\""))
            .await)
    }

    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toHexString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &format!("{value:x}")).await?;

        Ok(string.into())
    }
}

fn char_digit(ch: JavaChar) -> Option<i32> {
    match ch {
        ch if (b'0' as JavaChar..=b'9' as JavaChar).contains(&ch) => Some((ch - b'0' as JavaChar) as i32),
        ch if (b'a' as JavaChar..=b'z' as JavaChar).contains(&ch) => Some((ch - b'a' as JavaChar) as i32 + 10),
        ch if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&ch) => Some((ch - b'A' as JavaChar) as i32 + 10),
        _ => None,
    }
}
