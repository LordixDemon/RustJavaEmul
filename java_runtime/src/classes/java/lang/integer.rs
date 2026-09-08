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
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
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
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, Default::default()),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, Default::default()),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("longValue", "()J", Self::long_value, Default::default()),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, Default::default()),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "(I)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toString", "(II)Ljava/lang/String;", Self::to_string_radix, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "toBinaryString",
                    "(I)Ljava/lang/String;",
                    Self::to_binary_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("toHexString", "(I)Ljava/lang/String;", Self::to_hex_string, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toOctalString", "(I)Ljava/lang/String;", Self::to_octal_string, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;I)Ljava/lang/Integer;",
                    Self::value_of_string_radix,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Integer;)I", Self::compare_to, Default::default()),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/Integer;",
                    Self::decode,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("highestOneBit", "(I)I", Self::highest_one_bit, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "I", Default::default()),
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "I",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "I",
                    java_constants::FieldAccessFlags::STATIC | java_constants::FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.lang.Integer::<init>({:?}, {:?})", &this, value);

        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "I", value).await?;

        Ok(())
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Integer", "MIN_VALUE", "I", i32::MIN).await?;
        jvm.put_static_field("java/lang/Integer", "MAX_VALUE", "I", i32::MAX).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? as i8)
    }

    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? as i16)
    }

    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? as i64)
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? as f32)
    }

    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? as f64)
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

    async fn to_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: i32, radix: i32) -> Result<ClassInstanceRef<String>> {
        if !(2..=36).contains(&radix) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "radix out of range").await);
        }
        Ok(JavaLangString::from_rust_string(jvm, &to_radix_string(value, radix)).await?.into())
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

    async fn init_string(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        let value = Self::parse_int(jvm, context, s).await?;
        Self::init(jvm, context, this, value).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        if other.is_null() || other.class_definition().name() != "java/lang/Integer" {
            return Ok(false);
        }
        Ok(jvm.get_field::<i32>(&this, "value", "I").await? == jvm.get_field::<i32>(&other, "value", "I").await?)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "value", "I").await
    }

    async fn value_of_string_radix(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        s: ClassInstanceRef<String>,
        radix: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let value = Self::parse_int_radix(jvm, context, s, radix).await?;
        Self::value_of(jvm, context, value).await
    }

    async fn to_binary_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value:b}")).await?.into())
    }

    async fn to_octal_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value:o}")).await?.into())
    }

    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toHexString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &format!("{value:x}")).await?;

        Ok(string.into())
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let this_value: i32 = jvm.get_field(&this, "value", "I").await?;
        let other_value: i32 = jvm.get_field(&other, "value", "I").await?;
        Ok(match this_value.cmp(&other_value) {
            core::cmp::Ordering::Less => -1,
            core::cmp::Ordering::Equal => 0,
            core::cmp::Ordering::Greater => 1,
        })
    }

    async fn decode(jvm: &Jvm, context: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if s.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let raw = JavaLangString::to_rust_string(jvm, &s).await?;
        let (negative, rest) = if let Some(stripped) = raw.strip_prefix('-') {
            (true, stripped)
        } else if let Some(stripped) = raw.strip_prefix('+') {
            (false, stripped)
        } else {
            (false, raw.as_str())
        };
        let (radix, digits) = if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
            (16, hex)
        } else if let Some(hex) = rest.strip_prefix('#') {
            (16, hex)
        } else if rest.len() > 1 && rest.starts_with('0') {
            (8, rest)
        } else {
            (10, rest)
        };
        let java_digits = JavaLangString::from_rust_string(jvm, digits).await?;
        let mut value = Self::parse_int_radix(jvm, context, java_digits.into(), radix).await?;
        if negative {
            value = value.wrapping_neg();
        }
        Self::value_of(jvm, context, value).await
    }

    async fn highest_one_bit(_: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<i32> {
        if value == 0 {
            return Ok(0);
        }
        Ok(1i32.wrapping_shl(31 - value.leading_zeros()))
    }
}

fn to_radix_string(value: i32, radix: i32) -> alloc::string::String {
    if radix == 10 {
        return value.to_string();
    }
    const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut n = value as i64;
    let negative = n < 0;
    if negative {
        n = -n;
    }
    let mut buf = alloc::vec::Vec::new();
    if n == 0 {
        buf.push(b'0');
    }
    while n > 0 {
        buf.push(DIGITS[(n % radix as i64) as usize]);
        n /= radix as i64;
    }
    if negative {
        buf.push(b'-');
    }
    buf.reverse();
    alloc::string::String::from_utf8(buf).unwrap_or_default()
}

fn char_digit(ch: JavaChar) -> Option<i32> {
    match ch {
        ch if (b'0' as JavaChar..=b'9' as JavaChar).contains(&ch) => Some((ch - b'0' as JavaChar) as i32),
        ch if (b'a' as JavaChar..=b'z' as JavaChar).contains(&ch) => Some((ch - b'a' as JavaChar) as i32 + 10),
        ch if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&ch) => Some((ch - b'A' as JavaChar) as i32 + 10),
        _ => None,
    }
}
