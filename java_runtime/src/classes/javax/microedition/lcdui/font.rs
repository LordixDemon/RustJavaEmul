use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class javax.microedition.lcdui.Font
pub struct Font;

impl Font {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Font",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(III)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getFont",
                    "(III)Ljavax/microedition/lcdui/Font;",
                    Self::get_font,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getDefaultFont",
                    "()Ljavax/microedition/lcdui/Font;",
                    Self::get_default_font,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getFace", "()I", Self::get_face, Default::default()),
                JavaMethodProto::new("getStyle", "()I", Self::get_style, Default::default()),
                JavaMethodProto::new("getSize", "()I", Self::get_size, Default::default()),
                JavaMethodProto::new("isPlain", "()Z", Self::is_plain, Default::default()),
                JavaMethodProto::new("isBold", "()Z", Self::is_bold, Default::default()),
                JavaMethodProto::new("isItalic", "()Z", Self::is_italic, Default::default()),
                JavaMethodProto::new("isUnderlined", "()Z", Self::is_underlined, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("getBaselinePosition", "()I", Self::get_baseline_position, Default::default()),
                JavaMethodProto::new("charWidth", "(C)I", Self::char_width, Default::default()),
                JavaMethodProto::new("charsWidth", "([CII)I", Self::chars_width, Default::default()),
                JavaMethodProto::new("stringWidth", "(Ljava/lang/String;)I", Self::string_width, Default::default()),
                JavaMethodProto::new("substringWidth", "(Ljava/lang/String;II)I", Self::substring_width, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("face", "I", Default::default()),
                JavaFieldProto::new("style", "I", Default::default()),
                JavaFieldProto::new("size", "I", Default::default()),
                JavaFieldProto::new("FACE_SYSTEM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FACE_MONOSPACE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FACE_PROPORTIONAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STYLE_PLAIN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STYLE_BOLD", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STYLE_ITALIC", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STYLE_UNDERLINED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SIZE_SMALL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SIZE_MEDIUM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SIZE_LARGE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Font";
        jvm.put_static_field(class, "FACE_SYSTEM", "I", 0).await?;
        jvm.put_static_field(class, "FACE_MONOSPACE", "I", 32).await?;
        jvm.put_static_field(class, "FACE_PROPORTIONAL", "I", 64).await?;
        jvm.put_static_field(class, "STYLE_PLAIN", "I", 0).await?;
        jvm.put_static_field(class, "STYLE_BOLD", "I", 1).await?;
        jvm.put_static_field(class, "STYLE_ITALIC", "I", 2).await?;
        jvm.put_static_field(class, "STYLE_UNDERLINED", "I", 4).await?;
        jvm.put_static_field(class, "SIZE_SMALL", "I", 8).await?;
        jvm.put_static_field(class, "SIZE_MEDIUM", "I", 0).await?;
        jvm.put_static_field(class, "SIZE_LARGE", "I", 16).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, face: i32, style: i32, size: i32) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Font::<init>({this:?}, {face:?}, {style:?}, {size:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "face", "I", face).await?;
        jvm.put_field(&mut this, "style", "I", style).await?;
        jvm.put_field(&mut this, "size", "I", size).await?;

        Ok(())
    }

    async fn get_font(jvm: &Jvm, _: &mut RuntimeContext, face: i32, style: i32, size: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("javax.microedition.lcdui.Font::getFont({face:?}, {style:?}, {size:?})");

        Ok(jvm
            .new_class("javax/microedition/lcdui/Font", "(III)V", (face, style, size))
            .await?
            .into())
    }

    async fn get_default_font(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("javax/microedition/lcdui/Font", "(III)V", (0, 0, 0)).await?.into())
    }

    async fn get_face(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "face", "I").await
    }

    async fn get_style(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "style", "I").await
    }

    async fn get_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "size", "I").await
    }

    async fn is_plain(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let style: i32 = jvm.get_field(&this, "style", "I").await?;
        Ok(style == 0)
    }

    async fn is_bold(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let style: i32 = jvm.get_field(&this, "style", "I").await?;
        Ok((style & 1) != 0)
    }

    async fn is_italic(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let style: i32 = jvm.get_field(&this, "style", "I").await?;
        Ok((style & 2) != 0)
    }

    async fn is_underlined(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let style: i32 = jvm.get_field(&this, "style", "I").await?;
        Ok((style & 4) != 0)
    }

    async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Font::getHeight({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Ok(Self::height_for_size(size))
    }

    async fn get_baseline_position(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.lcdui.Font::getBaselinePosition({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Ok(Self::baseline_for_size(size))
    }

    async fn char_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _char: JavaChar) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Font::charWidth({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Ok(Self::advance_for_size(size))
    }

    async fn chars_width(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Font::charsWidth({this:?}, {chars:?}, {offset:?}, {length:?})");

        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
        }

        let array_length = jvm.array_length(&chars).await?;
        Self::check_range(jvm, array_length, offset, length).await?;
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Ok(length * Self::advance_for_size(size))
    }

    async fn string_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Font::stringWidth({this:?}, {string:?})");

        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let text = JavaLangString::to_rust_string(jvm, &string).await?;
        Ok(text.chars().count() as i32 * Self::advance_for_size(size))
    }

    async fn substring_width(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Font::substringWidth({this:?}, {string:?}, {offset:?}, {length:?})");

        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
        let string_length = jvm.array_length(&chars).await?;
        Self::check_range(jvm, string_length, offset, length).await?;
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        Ok(length * Self::advance_for_size(size))
    }

    async fn check_range(jvm: &Jvm, total_length: usize, offset: i32, length: i32) -> Result<()> {
        let valid =
            offset >= 0 && length >= 0 && (offset as usize) <= total_length && (length as usize) <= total_length.saturating_sub(offset as usize);
        if valid {
            Ok(())
        } else {
            Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid font text range").await)
        }
    }

    pub(crate) fn height_for_size(size: i32) -> i32 {
        match size {
            8 => 8,
            16 => 16,
            _ => 12,
        }
    }

    pub(crate) fn advance_for_size(size: i32) -> i32 {
        match size {
            8 => 5,
            16 => 12,
            _ => 6,
        }
    }

    pub(crate) fn baseline_for_size(size: i32) -> i32 {
        match size {
            8 => 7,
            16 => 14,
            _ => 9,
        }
    }

    pub(crate) fn pixel_scale_for_size(size: i32) -> i32 {
        match size {
            16 => 2,
            _ => 1,
        }
    }
}
