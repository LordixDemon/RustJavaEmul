#[allow(unused_imports)]
use super::*;
use crate::RuntimeContext;
use alloc::{
    string::{String as RustString, ToString},
    vec::Vec,
};
use bytemuck::{cast_slice, cast_vec};
#[allow(unused_imports)]
use core::cmp::Ordering;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

impl String {
    pub(super) async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, ch);

        jvm.invoke_virtual(&this, "indexOf", "(II)I", (ch, 0)).await
    }

    pub(super) async fn index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?}, {:?})", &this, ch, from_index);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let index = this_string
            .chars()
            .skip(from_index as usize)
            .position(|x| x as u32 == ch as u32)
            .map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    pub(super) async fn index_of_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, &str);

        jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/String;I)I", (str, 0)).await
    }

    pub(super) async fn index_of_string_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?}, {})", &this, &str, from_index);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let str_string = JavaLangString::to_rust_string(jvm, &str.clone()).await?;

        tracing::trace!("this_string: {:?}", this_string);
        tracing::trace!("str_string: {:?}", str_string);

        let chars = this_string.chars().skip(from_index as usize).collect::<Vec<_>>();
        let str_chars = str_string.chars().collect::<Vec<_>>();
        let index = chars.windows(str_chars.len()).position(|x| x == str_chars).map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    pub(super) async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({:?}, {:?})", &this, ch);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let index = this_string
            .chars()
            .collect::<Vec<_>>() // TODO i think we don't need collect..
            .into_iter()
            .rposition(|x| x as u32 == ch as u32)
            .map(|x| x as i32);

        Ok(index.unwrap_or(-1))
    }

    pub(super) async fn trim(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::trim({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let trimmed = string.trim_matches(|c: char| c <= ' ').to_string();

        Ok(JavaLangString::from_rust_string(jvm, &trimmed).await?.into()) // TODO buffer sharing
    }

    pub(super) async fn to_upper_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toUpperCase({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let upper = string.to_uppercase().to_string();

        Ok(JavaLangString::from_rust_string(jvm, &upper).await?.into())
    }

    pub(super) async fn starts_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, prefix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({:?}, {:?})", &this, &prefix);

        jvm.invoke_virtual(&this, "startsWith", "(Ljava/lang/String;I)Z", (prefix, 0)).await
    }

    pub(super) async fn starts_with_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        prefix: ClassInstanceRef<Self>,
        offset: i32,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({:?}, {:?}, {})", &this, &prefix, offset);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone())
            .await?
            .chars()
            .skip(offset as usize)
            .collect::<RustString>();
        let prefix_string = JavaLangString::to_rust_string(jvm, &prefix.clone()).await?;

        Ok(this_string.starts_with(&prefix_string))
    }

    pub(super) async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("C", 0).await?;
        jvm.put_field(&mut this, "value", "[C", array).await?;

        Ok(())
    }

    pub(super) async fn init_with_byte_array_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {:?})", &this, &value, &charset_name);

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/String",
                "<init>",
                "([BIILjava/lang/String;)V",
                (value, 0, count, charset_name),
            )
            .await?;

        Ok(())
    }

    pub(super) async fn init_with_partial_byte_array_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        offset: i32,
        count: i32,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!(
            "java.lang.String::<init>({:?}, {:?}, {}, {}, {:?})",
            &this,
            &value,
            offset,
            count,
            &charset_name
        );

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    pub(super) async fn equals_ignore_case(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::equalsIgnoreCase({:?}, {:?})", &this, &other);

        if other.is_null() {
            return Ok(false);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;

        Ok(this_string.eq_ignore_ascii_case(&other_string) || this_string.to_lowercase() == other_string.to_lowercase())
    }

    pub(super) async fn get_bytes_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({:?}, {:?})", &this, &charset_name);

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;

        let bytes = cast_vec(Self::encode_str(&charset, &string));

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut byte_array).await?.write(0, &bytes)?;

        Ok(byte_array.into())
    }

    pub(super) async fn to_lower_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toLowerCase({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let lower = string.to_lowercase();

        Ok(JavaLangString::from_rust_string(jvm, &lower).await?.into())
    }

    pub(super) async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        old_char: JavaChar,
        new_char: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::replace({:?}, {}, {})", &this, old_char, new_char);

        let value = jvm.get_field(&this, "value", "[C").await?;
        let length = jvm.array_length(&value).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, 0, length).await?;

        let replaced: Vec<JavaChar> = chars.into_iter().map(|c| if c == old_char { new_char } else { c }).collect();

        let mut array = jvm.instantiate_array("C", replaced.len()).await?;
        jvm.store_array(&mut array, 0, replaced).await?;

        let new_string = jvm.new_class("java/lang/String", "([C)V", (array,)).await?;

        Ok(new_string.into())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn region_matches(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        ignore_case: bool,
        toffset: i32,
        other: ClassInstanceRef<Self>,
        ooffset: i32,
        len: i32,
    ) -> Result<bool> {
        tracing::debug!(
            "java.lang.String::regionMatches({:?}, {}, {}, {:?}, {}, {})",
            &this,
            ignore_case,
            toffset,
            &other,
            ooffset,
            len
        );

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other is null").await);
        }

        if toffset < 0 || ooffset < 0 || len < 0 {
            return Ok(false);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;

        let this_chars: Vec<u16> = this_string.encode_utf16().collect();
        let other_chars: Vec<u16> = other_string.encode_utf16().collect();

        let end_t = toffset as usize + len as usize;
        let end_o = ooffset as usize + len as usize;
        if end_t > this_chars.len() || end_o > other_chars.len() {
            return Ok(false);
        }

        let this_slice = &this_chars[toffset as usize..end_t];
        let other_slice = &other_chars[ooffset as usize..end_o];

        if ignore_case {
            let to_lower = |c: u16| -> u16 {
                char::from_u32(c as u32)
                    .map(|ch| ch.to_lowercase().next().unwrap_or(ch) as u32 as u16)
                    .unwrap_or(c)
            };
            Ok(this_slice.iter().copied().map(to_lower).eq(other_slice.iter().copied().map(to_lower)))
        } else {
            Ok(this_slice == other_slice)
        }
    }

    pub(super) async fn last_index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({:?}, {}, {})", &this, ch, from_index);

        if from_index < 0 {
            return Ok(-1);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let chars: Vec<char> = this_string.chars().collect();
        let end = (from_index as usize + 1).min(chars.len());

        let index = chars[..end].iter().rposition(|&c| c as u32 == ch as u32).map(|x| x as i32);

        Ok(index.unwrap_or(-1))
    }

    pub(super) async fn last_index_of_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
    ) -> Result<i32> {
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        jvm.invoke_virtual(&this, "lastIndexOf", "(Ljava/lang/String;I)I", (str, this_string.chars().count() as i32))
            .await
    }

    pub(super) async fn last_index_of_string_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        if str.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        if from_index < 0 {
            return Ok(-1);
        }
        let this_chars: Vec<char> = JavaLangString::to_rust_string(jvm, &this).await?.chars().collect();
        let needle: Vec<char> = JavaLangString::to_rust_string(jvm, &str).await?.chars().collect();
        if needle.is_empty() {
            return Ok((from_index as usize).min(this_chars.len()) as i32);
        }
        if this_chars.len() < needle.len() {
            return Ok(-1);
        }
        let max_start = (from_index as usize).min(this_chars.len() - needle.len());
        for start in (0..=max_start).rev() {
            if this_chars[start..start + needle.len()] == needle[..] {
                return Ok(start as i32);
            }
        }
        Ok(-1)
    }

    pub(super) async fn ends_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, suffix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::endsWith({:?}, {:?})", &this, &suffix);

        if suffix.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "suffix is null").await);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let suffix_string = JavaLangString::to_rust_string(jvm, &suffix).await?;

        Ok(this_string.ends_with(&suffix_string))
    }

    pub(super) async fn intern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let text = JavaLangString::to_rust_string(jvm, &this).await?;
        Ok(JavaLangString::intern_rust_string(jvm, &text).await?.into())
    }

    pub(super) async fn value_of_boolean(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = if value { "true" } else { "false" };
        Ok(JavaLangString::from_rust_string(jvm, string).await?.into())
    }

    pub(super) async fn value_of_long(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    pub(super) async fn value_of_float(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    pub(super) async fn value_of_double(jvm: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    pub(super) async fn value_of_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        value: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({:?})", &value);

        let new_string = jvm.new_class("java/lang/String", "([C)V", (value,)).await?;

        Ok(new_string.into())
    }

    pub(super) async fn value_of_partial_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        value: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        count: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({:?}, {}, {})", &value, offset, count);

        let new_string = jvm.new_class("java/lang/String", "([CII)V", (value, offset, count)).await?;

        Ok(new_string.into())
    }
}
