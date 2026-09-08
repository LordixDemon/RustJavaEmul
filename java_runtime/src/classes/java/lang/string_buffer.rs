use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.lang.StringBuffer
pub struct StringBuffer;

impl StringBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/StringBuffer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_buffer_length, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, Default::default()),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/Object;)Ljava/lang/StringBuffer;",
                    Self::append_object,
                    Default::default(),
                ),
                JavaMethodProto::new("append", "(Z)Ljava/lang/StringBuffer;", Self::append_boolean, Default::default()),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuffer;", Self::append_integer, Default::default()),
                JavaMethodProto::new("append", "(J)Ljava/lang/StringBuffer;", Self::append_long, Default::default()),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuffer;", Self::append_character, Default::default()),
                JavaMethodProto::new("append", "(F)Ljava/lang/StringBuffer;", Self::append_float, Default::default()),
                JavaMethodProto::new("append", "(D)Ljava/lang/StringBuffer;", Self::append_double, Default::default()),
                JavaMethodProto::new("append", "([CII)Ljava/lang/StringBuffer;", Self::append_char_array, Default::default()),
                JavaMethodProto::new("append", "([C)Ljava/lang/StringBuffer;", Self::append_chars, Default::default()),
                JavaMethodProto::new("capacity", "()I", Self::capacity, Default::default()),
                JavaMethodProto::new("ensureCapacity", "(I)V", Self::ensure_capacity_public, Default::default()),
                JavaMethodProto::new("reverse", "()Ljava/lang/StringBuffer;", Self::reverse, Default::default()),
                JavaMethodProto::new("getChars", "(II[CI)V", Self::get_chars, Default::default()),
                JavaMethodProto::new("delete", "(II)Ljava/lang/StringBuffer;", Self::delete, Default::default()),
                JavaMethodProto::new("deleteCharAt", "(I)Ljava/lang/StringBuffer;", Self::delete_char_at, Default::default()),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::insert_string,
                    Default::default(),
                ),
                JavaMethodProto::new("insert", "(IC)Ljava/lang/StringBuffer;", Self::insert_char, Default::default()),
                JavaMethodProto::new("insert", "(I[C)Ljava/lang/StringBuffer;", Self::insert_chars, Default::default()),
                JavaMethodProto::new("insert", "(II)Ljava/lang/StringBuffer;", Self::insert_int, Default::default()),
                JavaMethodProto::new("insert", "(IJ)Ljava/lang/StringBuffer;", Self::insert_long, Default::default()),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/Object;)Ljava/lang/StringBuffer;",
                    Self::insert_object,
                    Default::default(),
                ),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("setLength", "(I)V", Self::set_length, Default::default()),
                JavaMethodProto::new("length", "()I", Self::length, Default::default()),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, Default::default()),
                JavaMethodProto::new("setCharAt", "(IC)V", Self::set_char_at, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/StringBuffer", "<init>", "(I)V", (16,)).await?;

        Ok(())
    }

    async fn init_with_buffer_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, length: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?}, {:?})", &this, length);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("C", length as _).await?;
        jvm.put_field(&mut this, "value", "[C", array).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?}, {:?})", &this, &string,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let value_array = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&value_array).await? as i32;

        jvm.put_field(&mut this, "value", "[C", value_array).await?;
        jvm.put_field(&mut this, "count", "I", length).await?;

        Ok(())
    }

    async fn append_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, &string,);

        let string = if string.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &string).await?
        };

        Self::append(jvm, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, &object,);

        let string = if object.is_null() {
            "null".into()
        } else {
            let string = jvm.invoke_virtual(&object, "toString", "()Ljava/lang/String;", ()).await?;
            JavaLangString::to_rust_string(jvm, &string).await?
        };

        Self::append(jvm, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_integer(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_boolean(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let value = value.to_string();

        Self::append(jvm, &mut this, &value).await?;

        Ok(this)
    }

    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_character(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: u16) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let value = RustString::from_utf16(&[value]).unwrap();

        Self::append(jvm, &mut this, &value).await?;

        Ok(this)
    }

    async fn append_float(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<ClassInstanceRef<Self>> {
        Self::append(jvm, &mut this, &value.to_string()).await?;
        Ok(this)
    }

    async fn append_double(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<Self>> {
        Self::append(jvm, &mut this, &value.to_string()).await?;
        Ok(this)
    }

    async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        src_begin: i32,
        src_end: i32,
        mut dst: ClassInstanceRef<Array<JavaChar>>,
        dst_begin: i32,
    ) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let begin = src_begin.max(0) as usize;
        let end = src_end.clamp(src_begin, count) as usize;
        if begin >= end {
            return Ok(());
        }
        let java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&java_value, begin, end - begin).await?;
        jvm.store_array(&mut dst, dst_begin.max(0) as usize, chars).await
    }

    async fn append_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?}, {:?}, {:?})", &this, &array, offset, length);

        let value: Vec<JavaChar> = jvm.load_array(&array, offset as _, length as _).await?;
        let string = RustString::from_utf16(&value).unwrap();

        Self::append(jvm, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_chars(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str").await);
        }
        let length = jvm.array_length(&array).await? as i32;
        Self::append_char_array(jvm, context, this, array, 0, length).await
    }

    async fn capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let java_value = jvm.get_field(&this, "value", "[C").await?;
        Ok(jvm.array_length(&java_value).await? as i32)
    }

    async fn ensure_capacity_public(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, minimum: i32) -> Result<()> {
        Self::ensure_capacity(jvm, &mut this, minimum.max(0) as usize).await
    }

    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let mut java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let mut chars: Vec<JavaChar> = jvm.load_array(&java_value, 0, count as usize).await?;
        chars.reverse();
        jvm.store_array(&mut java_value, 0, chars).await?;
        Ok(this)
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::delete({this:?}, {start}, {end})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        let mut java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&java_value, 0, count as _).await?;

        let new_chars = chars
            .iter()
            .take(start as _)
            .chain(chars.iter().skip(end as _))
            .cloned()
            .collect::<Vec<_>>();
        let new_count = new_chars.len() as i32;

        jvm.store_array(&mut java_value, 0, new_chars).await?;
        jvm.put_field(&mut this, "count", "I", new_count).await?;

        Ok(this)
    }

    async fn delete_char_at(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::deleteCharAt({this:?}, {index})");

        Self::delete(jvm, context, this, index, index + 1).await
    }

    async fn insert_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {string:?})");

        let string = if string.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &string).await?
        };

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("offset {offset}, length {count}"))
                .await);
        }

        let value_to_insert = string.encode_utf16().collect::<Vec<_>>();
        let insert_len = value_to_insert.len();
        Self::ensure_capacity(jvm, &mut this, count as usize + insert_len).await?;

        let mut java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let old_chars: Vec<JavaChar> = jvm.load_array(&java_value, 0, count as _).await?;
        let mut new_chars = Vec::with_capacity(count as usize + insert_len);
        new_chars.extend_from_slice(&old_chars[..offset as usize]);
        new_chars.extend(value_to_insert);
        new_chars.extend_from_slice(&old_chars[offset as usize..]);

        jvm.store_array(&mut java_value, 0, new_chars).await?;
        jvm.put_field(&mut this, "count", "I", count + insert_len as i32).await?;

        Ok(this)
    }

    async fn insert_char(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        ch: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        let rust = char::from_u32(ch as u32).unwrap_or('\u{FFFD}').to_string();
        let java = JavaLangString::from_rust_string(jvm, &rust).await?;
        Self::insert_string(jvm, context, this, offset, java.into()).await
    }

    async fn insert_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        chars: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str").await);
        }
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("offset {offset}, length {count}"))
                .await);
        }
        let insert_len = jvm.array_length(&chars).await?;
        let to_insert: Vec<JavaChar> = jvm.load_array(&chars, 0, insert_len).await?;

        Self::ensure_capacity(jvm, &mut this, count as usize + insert_len).await?;

        let mut java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let old_chars: Vec<JavaChar> = jvm.load_array(&java_value, 0, count as _).await?;
        let mut new_chars = Vec::with_capacity(count as usize + insert_len);
        new_chars.extend_from_slice(&old_chars[..offset as usize]);
        new_chars.extend(to_insert);
        new_chars.extend_from_slice(&old_chars[offset as usize..]);

        jvm.store_array(&mut java_value, 0, new_chars).await?;
        jvm.put_field(&mut this, "count", "I", count + insert_len as i32).await?;

        Ok(this)
    }

    async fn insert_int(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let java = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        Self::insert_string(jvm, context, this, offset, java.into()).await
    }

    async fn insert_long(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: i64,
    ) -> Result<ClassInstanceRef<Self>> {
        let java = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        Self::insert_string(jvm, context, this, offset, java.into()).await
    }

    async fn insert_object(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        obj: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string = if obj.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?
        } else {
            jvm.invoke_virtual(&obj, "toString", "()Ljava/lang/String;", ()).await?
        };
        Self::insert_string(jvm, context, this, offset, string.into()).await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({:?})", &this);

        let java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        let string = jvm.new_class("java/lang/String", "([CII)V", (java_value, 0, count)).await?;

        Ok(string.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, capacity: usize) -> Result<()> {
        let java_value_array = jvm.get_field(this, "value", "[C").await?;
        let current_capacity = jvm.array_length(&java_value_array).await?;

        if current_capacity < capacity {
            let old_values: Vec<JavaChar> = jvm.load_array(&java_value_array, 0, current_capacity).await?;
            let new_capacity = capacity * 2;

            let mut java_new_value_array = jvm.instantiate_array("C", new_capacity).await?;
            jvm.put_field(this, "value", "[C", java_new_value_array.clone()).await?;
            jvm.store_array(&mut java_new_value_array, 0, old_values).await?;
        }

        Ok(())
    }

    async fn append(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, string: &str) -> Result<()> {
        let current_count: i32 = jvm.get_field(this, "count", "I").await?;

        let value_to_add = string.encode_utf16().collect::<Vec<_>>();
        let count_to_add = value_to_add.len() as i32;

        StringBuffer::ensure_capacity(jvm, this, (current_count + count_to_add) as _).await?;

        let mut java_value_array = jvm.get_field(this, "value", "[C").await?;
        jvm.store_array(&mut java_value_array, current_count as _, value_to_add).await?;
        jvm.put_field(this, "count", "I", current_count + count_to_add).await?;

        Ok(())
    }

    async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, length: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::setLength({:?}, {:?})", &this, length);

        jvm.put_field(&mut this, "count", "I", length).await?;

        Ok(())
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.StringBuffer::length({:?})", &this);

        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        Ok(count)
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<JavaChar> {
        tracing::debug!("java.lang.StringBuffer::charAt({:?}, {:?})", &this, index);

        let java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let char_at: JavaChar = jvm.load_array(&java_value, index as _, 1).await?.into_iter().next().unwrap();

        Ok(char_at)
    }

    async fn set_char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, ch: JavaChar) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("index {index}, length {count}"))
                .await);
        }
        let mut java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        jvm.store_array(&mut java_value, index as usize, vec![ch]).await
    }
}
