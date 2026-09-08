use super::super::StringBuffer;
#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, System},
};
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use bytemuck::{cast_slice, cast_vec};
#[allow(unused_imports)]
use core::cmp::Ordering;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

impl String {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/String",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "([B)V", Self::init_with_byte_array, Default::default()),
                JavaMethodProto::new("<init>", "([C)V", Self::init_with_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_partial_byte_array, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "([BLjava/lang/String;)V",
                    Self::init_with_byte_array_charset,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "([BIILjava/lang/String;)V",
                    Self::init_with_partial_byte_array_charset,
                    Default::default(),
                ),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/StringBuffer;)V", Self::init_with_string_buffer, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("equalsIgnoreCase", "(Ljava/lang/String;)Z", Self::equals_ignore_case, Default::default()),
                JavaMethodProto::new("compareTo", "(Ljava/lang/String;)I", Self::compare_to, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, Default::default()),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, Default::default()),
                JavaMethodProto::new("getBytes", "(Ljava/lang/String;)[B", Self::get_bytes_charset, Default::default()),
                JavaMethodProto::new("getChars", "(II[CI)V", Self::get_chars, Default::default()),
                JavaMethodProto::new("toCharArray", "()[C", Self::to_char_array, Default::default()),
                JavaMethodProto::new("toUpperCase", "()Ljava/lang/String;", Self::to_upper_case, Default::default()),
                JavaMethodProto::new("toLowerCase", "()Ljava/lang/String;", Self::to_lower_case, Default::default()),
                JavaMethodProto::new("length", "()I", Self::length, Default::default()),
                JavaMethodProto::new("concat", "(Ljava/lang/String;)Ljava/lang/String;", Self::concat, Default::default()),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, Default::default()),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_with_end, Default::default()),
                JavaMethodProto::new("replace", "(CC)Ljava/lang/String;", Self::replace, Default::default()),
                JavaMethodProto::new("regionMatches", "(ZILjava/lang/String;II)Z", Self::region_matches, Default::default()),
                JavaMethodProto::new("valueOf", "(Z)Ljava/lang/String;", Self::value_of_boolean, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(C)Ljava/lang/String;", Self::value_of_char, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/String;", Self::value_of_integer, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(J)Ljava/lang/String;", Self::value_of_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(F)Ljava/lang/String;", Self::value_of_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(D)Ljava/lang/String;", Self::value_of_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "([C)Ljava/lang/String;", Self::value_of_char_array, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "([CII)Ljava/lang/String;",
                    Self::value_of_partial_char_array,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/Object;)Ljava/lang/String;",
                    Self::value_of_object,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("indexOf", "(I)I", Self::index_of, Default::default()),
                JavaMethodProto::new("indexOf", "(II)I", Self::index_of_from, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;)I", Self::index_of_string, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_string_from, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(I)I", Self::last_index_of, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(II)I", Self::last_index_of_from, Default::default()),
                JavaMethodProto::new("trim", "()Ljava/lang/String;", Self::trim, Default::default()),
                JavaMethodProto::new("startsWith", "(Ljava/lang/String;)Z", Self::starts_with, Default::default()),
                JavaMethodProto::new("startsWith", "(Ljava/lang/String;I)Z", Self::starts_with_offset, Default::default()),
                JavaMethodProto::new("endsWith", "(Ljava/lang/String;)Z", Self::ends_with, Default::default()),
                JavaMethodProto::new("intern", "()Ljava/lang/String;", Self::intern, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/String;)I", Self::last_index_of_string, Default::default()),
                JavaMethodProto::new(
                    "lastIndexOf",
                    "(Ljava/lang/String;I)I",
                    Self::last_index_of_string_from,
                    Default::default(),
                ),
                JavaMethodProto::new("contains", "(Ljava/lang/CharSequence;)Z", Self::contains, Default::default()),
                JavaMethodProto::new(
                    "compareToIgnoreCase",
                    "(Ljava/lang/String;)I",
                    Self::compare_to_ignore_case,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "toLowerCase",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::to_lower_case_locale,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "toUpperCase",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::to_upper_case_locale,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "replace",
                    "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;",
                    Self::replace_sequences,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                    Self::format,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                    Self::format_locale,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![JavaFieldProto::new("value", "[C", Default::default())],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init_with_byte_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "([BII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    pub(super) async fn init_with_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "([CII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    pub(super) async fn init_with_partial_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let mut array = jvm.instantiate_array("C", count as _).await?;
        jvm.put_field(&mut this, "value", "[C", array.clone()).await?;

        let data: Vec<JavaChar> = jvm.load_array(&value, offset as _, count as _).await?;
        jvm.store_array(&mut array, 0, data).await?; // TODO we should store value, offset, count like in java

        Ok(())
    }

    pub(super) async fn init_with_partial_byte_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = System::get_charset(jvm).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    pub(super) async fn init_with_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&value, "toCharArray", "()[C", ()).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", (chars,)).await?;

        Ok(())
    }

    pub(super) async fn init_with_string_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<StringBuffer>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let string: ClassInstanceRef<Self> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(Ljava/lang/String;)V", (string,))
            .await?;

        Ok(())
    }

    pub(super) async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::equals({:?}, {:?})", &this, &other);

        if other.is_null() {
            return Ok(false);
        }

        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;

        if this_string == other_string { Ok(true) } else { Ok(false) }
    }

    pub(super) async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::compareTo({:?}, {:?})", &this, &other);

        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;

        let compare_result = this_string.cmp(&other_string);

        match compare_result {
            Ordering::Less => Ok(-1),
            Ordering::Equal => Ok(0),
            Ordering::Greater => Ok(1),
        }
    }

    pub(super) async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::hashCode({:?})", &this);

        let chars = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, jvm.array_length(&chars).await? as _).await?;

        let hash = chars.iter().fold(0i32, |acc, &c| acc.wrapping_mul(31).wrapping_add(c as i32));

        Ok(hash)
    }

    pub(super) async fn to_string(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toString({:?})", &this);

        Ok(this)
    }

    pub(super) async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<u16> {
        tracing::debug!("java.lang.String::charAt({:?}, {})", &this, index);

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let length = jvm.array_length(&value).await? as i32;
        if index < 0 || index >= length {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("String index out of range: {index}"),
                )
                .await);
        }

        Ok(jvm.load_array(&value, index as usize, 1).await?[0])
    }

    pub(super) async fn concat(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::concat({:?}, {:?})", &this, &other);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other.clone()).await?;

        let concat = this_string + &other_string;

        Ok(JavaLangString::from_rust_string(jvm, &concat).await?.into())
    }

    pub(super) async fn get_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let charset = System::get_charset(jvm).await?;
        let bytes = cast_vec(Self::encode_str(&charset, &string));

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut byte_array).await?.write(0, &bytes)?;

        Ok(byte_array.into())
    }

    pub(super) async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        src_begin: i32,
        src_end: i32,
        mut dst: ClassInstanceRef<Array<JavaChar>>,
        dst_begin: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.lang.String::getChars({:?}, {}, {}, {:?}, {})",
            &this,
            src_begin,
            src_end,
            &dst,
            dst_begin
        );

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let length = jvm.array_length(&value).await? as i32;
        if src_begin < 0 || src_end > length || src_begin > src_end || dst_begin < 0 {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("src_begin {src_begin}, src_end {src_end}, length {length}"),
                )
                .await);
        }

        let count = src_end - src_begin;
        let chars: Vec<JavaChar> = jvm.load_array(&value, src_begin as _, count as _).await?;
        jvm.store_array(&mut dst, dst_begin as _, chars).await?;

        Ok(())
    }

    pub(super) async fn to_char_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        tracing::debug!("java.lang.String::toCharArray({:?})", &this);

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let length = jvm.array_length(&value).await?;
        let data: Vec<JavaChar> = jvm.load_array(&value, 0, length).await?;

        let mut new_array = jvm.instantiate_array("C", length).await?;
        jvm.store_array(&mut new_array, 0, data).await?;

        Ok(new_array.into())
    }

    pub(super) async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::length({:?})", &this);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(jvm.array_length(&value).await? as _)
    }

    pub(super) async fn substring(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {})", &this, begin_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        // java string indices are in utf-16 code units
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len() as i32;
        if begin_index < 0 || begin_index > length {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, length {length}"),
                )
                .await);
        }

        let substr = RustString::from_utf16_lossy(&utf16[begin_index as usize..]); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    pub(super) async fn substring_with_end(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {}, {})", &this, begin_index, end_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        // java string indices are in utf-16 code units
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len() as i32;
        if begin_index < 0 || end_index > length || begin_index > end_index {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, end {end_index}, length {length}"),
                )
                .await);
        }

        let substr = RustString::from_utf16_lossy(&utf16[begin_index as usize..end_index as usize]); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    pub(super) async fn value_of_char(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = RustString::from_utf16(&[value]).unwrap();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    pub(super) async fn value_of_integer(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = value.to_string();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    pub(super) async fn value_of_object(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.String::valueOf({:?})", &value);

        Ok(if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?
        })
    }

    pub(super) async fn is_empty(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(Self::length(jvm, context, this).await? == 0)
    }

    pub(super) async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, seq: ClassInstanceRef<Object>) -> Result<bool> {
        if seq.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let needle: ClassInstanceRef<Self> = jvm.invoke_virtual(&seq, "toString", "()Ljava/lang/String;", ()).await?;
        let index: i32 = jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/String;)I", (needle,)).await?;
        Ok(index >= 0)
    }

    pub(super) async fn compare_to_ignore_case(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?.to_lowercase();
        let other_string = JavaLangString::to_rust_string(jvm, &other).await?.to_lowercase();
        Ok(match this_string.cmp(&other_string) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        })
    }

    pub(super) async fn to_lower_case_locale(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _locale: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::to_lower_case(jvm, context, this).await
    }

    pub(super) async fn to_upper_case_locale(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _locale: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::to_upper_case(jvm, context, this).await
    }

    pub(super) async fn replace_sequences(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Object>,
        replacement: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        if target.is_null() || replacement.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let target_string: ClassInstanceRef<Self> = jvm.invoke_virtual(&target, "toString", "()Ljava/lang/String;", ()).await?;
        let replacement_string: ClassInstanceRef<Self> = jvm.invoke_virtual(&replacement, "toString", "()Ljava/lang/String;", ()).await?;
        let replaced = this_string.replace(
            &JavaLangString::to_rust_string(jvm, &target_string).await?,
            &JavaLangString::to_rust_string(jvm, &replacement_string).await?,
        );
        Ok(JavaLangString::from_rust_string(jvm, &replaced).await?.into())
    }

    pub(super) async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        format: ClassInstanceRef<Self>,
        args: ClassInstanceRef<Array<ClassInstanceRef<Object>>>,
    ) -> Result<ClassInstanceRef<Self>> {
        if format.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let pattern = JavaLangString::to_rust_string(jvm, &format).await?;
        let mut values = Vec::new();
        if !args.is_null() {
            let length = jvm.array_length(&args).await?;
            let items: Vec<ClassInstanceRef<Object>> = jvm.load_array(&args, 0, length).await?;
            for item in items {
                values.push(object_as_rust_string(jvm, item).await?);
            }
        }
        Ok(JavaLangString::from_rust_string(jvm, &format_pattern(&pattern, &values)).await?.into())
    }

    pub(super) async fn format_locale(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        _locale: ClassInstanceRef<Object>,
        format: ClassInstanceRef<Self>,
        args: ClassInstanceRef<Array<ClassInstanceRef<Object>>>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::format(jvm, context, format, args).await
    }
}

async fn object_as_rust_string(jvm: &Jvm, value: ClassInstanceRef<Object>) -> Result<RustString> {
    if value.is_null() {
        return Ok("null".into());
    }
    let string: ClassInstanceRef<crate::classes::java::lang::String> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;
    if string.is_null() {
        return Ok("null".into());
    }
    JavaLangString::to_rust_string(jvm, &string).await
}

fn format_pattern(pattern: &str, args: &[RustString]) -> RustString {
    let mut out = RustString::new();
    let mut chars = pattern.chars().peekable();
    let mut arg_index = 0usize;
    while let Some(ch) = chars.next() {
        if ch != '%' {
            out.push(ch);
            continue;
        }
        match chars.peek().copied() {
            Some('%') => {
                chars.next();
                out.push('%');
            }
            Some('n') => {
                chars.next();
                out.push('\n');
            }
            _ => {
                while matches!(chars.peek().copied(), Some('+' | '-' | '#' | '0' | ' ' | ',')) {
                    chars.next();
                }
                while matches!(chars.peek().copied(), Some('0'..='9')) {
                    chars.next();
                }
                if chars.peek() == Some(&'.') {
                    chars.next();
                    while matches!(chars.peek().copied(), Some('0'..='9')) {
                        chars.next();
                    }
                }
                let _conv = chars.next();
                if let Some(value) = args.get(arg_index) {
                    out.push_str(value);
                    arg_index += 1;
                }
            }
        }
    }
    out
}
