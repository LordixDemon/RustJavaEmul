use alloc::{format, string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class javax.microedition.lcdui.TextField
pub struct TextField;

impl TextField {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/TextField",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Ljava/lang/String;II)V", Self::init, Default::default()),
                JavaMethodProto::new("delete", "(II)V", Self::delete, Default::default()),
                JavaMethodProto::new("getCaretPosition", "()I", Self::get_caret_position, Default::default()),
                JavaMethodProto::new("getChars", "([C)I", Self::get_chars, Default::default()),
                JavaMethodProto::new("getConstraints", "()I", Self::get_constraints, Default::default()),
                JavaMethodProto::new("getMaxSize", "()I", Self::get_max_size, Default::default()),
                JavaMethodProto::new("getString", "()Ljava/lang/String;", Self::get_string, Default::default()),
                JavaMethodProto::new("insert", "(Ljava/lang/String;I)V", Self::insert_string, Default::default()),
                JavaMethodProto::new("insert", "([CIII)V", Self::insert_chars, Default::default()),
                JavaMethodProto::new("setConstraints", "(I)V", Self::set_constraints, Default::default()),
                JavaMethodProto::new(
                    "setInitialInputMode",
                    "(Ljava/lang/String;)V",
                    Self::set_initial_input_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("setMaxSize", "(I)I", Self::set_max_size, Default::default()),
                JavaMethodProto::new("setString", "(Ljava/lang/String;)V", Self::set_string, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("text", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("maxSize", "I", Default::default()),
                JavaFieldProto::new("constraints", "I", Default::default()),
                JavaFieldProto::new("initialInputMode", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("ANY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("EMAILADDR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NUMERIC", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PHONENUMBER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("URL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DECIMAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PASSWORD", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("UNEDITABLE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SENSITIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NON_PREDICTIVE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INITIAL_CAPS_WORD", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("INITIAL_CAPS_SENTENCE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/TextField";
        jvm.put_static_field(class, "ANY", "I", 0).await?;
        jvm.put_static_field(class, "EMAILADDR", "I", 1).await?;
        jvm.put_static_field(class, "NUMERIC", "I", 2).await?;
        jvm.put_static_field(class, "PHONENUMBER", "I", 3).await?;
        jvm.put_static_field(class, "URL", "I", 4).await?;
        jvm.put_static_field(class, "DECIMAL", "I", 5).await?;
        jvm.put_static_field(class, "PASSWORD", "I", 0x1_0000).await?;
        jvm.put_static_field(class, "UNEDITABLE", "I", 0x2_0000).await?;
        jvm.put_static_field(class, "SENSITIVE", "I", 0x4_0000).await?;
        jvm.put_static_field(class, "NON_PREDICTIVE", "I", 0x8_0000).await?;
        jvm.put_static_field(class, "INITIAL_CAPS_WORD", "I", 0x10_0000).await?;
        jvm.put_static_field(class, "INITIAL_CAPS_SENTENCE", "I", 0x20_0000).await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        text: ClassInstanceRef<String>,
        max_size: i32,
        constraints: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::<init>({this:?}, {label:?}, {text:?}, {max_size:?}, {constraints:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await?;
        jvm.put_field(&mut this, "maxSize", "I", max_size.max(0)).await?;
        jvm.put_field(&mut this, "constraints", "I", constraints).await?;
        jvm.put_field(&mut this, "initialInputMode", "Ljava/lang/String;", ClassInstanceRef::<String>::new(None))
            .await?;
        Self::put_text(jvm, &mut this, Self::string_to_chars(jvm, &text).await?).await
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, offset: i32, length: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::delete({this:?}, {offset:?}, {length:?})");

        let mut chars = Self::text_chars(jvm, &this).await?;
        Self::check_range(jvm, chars.len(), offset, length).await?;
        chars.drain(offset as usize..offset as usize + length as usize);
        Self::put_text(jvm, &mut this, chars).await
    }

    async fn get_caret_position(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::text_chars(jvm, &this).await?.len() as i32)
    }

    async fn get_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut data: ClassInstanceRef<Array<JavaChar>>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.TextField::getChars({this:?}, {data:?})");

        if data.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "data is null").await);
        }

        let chars = Self::text_chars(jvm, &this).await?;
        let data_len = jvm.array_length(&data).await?;
        if data_len < chars.len() {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", "destination is too small")
                .await);
        }
        let count = chars.len();
        jvm.store_array(&mut data, 0, chars).await?;
        Ok(count as i32)
    }

    async fn get_constraints(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "constraints", "I").await
    }

    async fn get_max_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "maxSize", "I").await
    }

    async fn get_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "text", "Ljava/lang/String;").await
    }

    async fn insert_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        text: ClassInstanceRef<String>,
        position: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::insert({this:?}, {text:?}, {position:?})");

        let insert = Self::string_to_chars(jvm, &text).await?;
        Self::insert_text(jvm, &mut this, insert, position).await
    }

    async fn insert_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
        position: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::insert({this:?}, {data:?}, {offset:?}, {length:?}, {position:?})");

        if data.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "data is null").await);
        }

        let data_len = jvm.array_length(&data).await?;
        Self::check_range(jvm, data_len, offset, length).await?;
        let insert = jvm.load_array(&data, offset as usize, length as usize).await?;
        Self::insert_text(jvm, &mut this, insert, position).await
    }

    async fn set_constraints(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, constraints: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::setConstraints({this:?}, {constraints:?})");

        jvm.put_field(&mut this, "constraints", "I", constraints).await
    }

    async fn set_initial_input_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        input_mode: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::setInitialInputMode({this:?}, {input_mode:?})");

        jvm.put_field(&mut this, "initialInputMode", "Ljava/lang/String;", input_mode).await
    }

    async fn set_max_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, max_size: i32) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.TextField::setMaxSize({this:?}, {max_size:?})");

        let max_size = max_size.max(0);
        jvm.put_field(&mut this, "maxSize", "I", max_size).await?;
        let chars = Self::text_chars(jvm, &this).await?;
        Self::put_text(jvm, &mut this, chars).await?;
        Ok(max_size)
    }

    async fn set_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, text: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.TextField::setString({this:?}, {text:?})");

        Self::put_text(jvm, &mut this, Self::string_to_chars(jvm, &text).await?).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::text_chars(jvm, &this).await?.len() as i32)
    }

    async fn insert_text(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, insert: Vec<JavaChar>, position: i32) -> Result<()> {
        let mut chars = Self::text_chars(jvm, this).await?;
        if position < 0 || position as usize > chars.len() {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("position {position}, length {}", chars.len()),
                )
                .await);
        }

        chars.splice(position as usize..position as usize, insert);
        Self::put_text(jvm, this, chars).await
    }

    async fn text_chars(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<JavaChar>> {
        let text: ClassInstanceRef<String> = jvm.get_field(this, "text", "Ljava/lang/String;").await?;
        Self::string_to_chars(jvm, &text).await
    }

    async fn string_to_chars(jvm: &Jvm, text: &ClassInstanceRef<String>) -> Result<Vec<JavaChar>> {
        if text.is_null() {
            return Ok(Vec::new());
        }

        let rust_text = JavaLangString::to_rust_string(jvm, text).await?;
        Ok(rust_text.encode_utf16().collect())
    }

    async fn put_text(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut chars: Vec<JavaChar>) -> Result<()> {
        let max_size: i32 = jvm.get_field(this, "maxSize", "I").await?;
        let max_size = max_size.max(0) as usize;
        if chars.len() > max_size {
            chars.truncate(max_size);
        }

        let text = RustString::from_utf16_lossy(&chars);
        let text = JavaLangString::from_rust_string(jvm, &text).await?;
        jvm.put_field(this, "text", "Ljava/lang/String;", text).await
    }

    async fn check_range(jvm: &Jvm, total: usize, offset: i32, length: i32) -> Result<()> {
        let valid = offset >= 0 && length >= 0 && (offset as usize) <= total && (length as usize) <= total.saturating_sub(offset as usize);
        if valid {
            Ok(())
        } else {
            Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("offset {offset}, length {length}, total {total}"),
                )
                .await)
        }
    }
}
