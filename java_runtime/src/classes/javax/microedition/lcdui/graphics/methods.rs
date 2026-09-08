use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaValue, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Font, Image},
};

use super::{TARGET_IMAGE_DESC, TARGET_IMAGE_FIELD, TextMetrics};

impl super::Graphics {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Graphics",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljavax/microedition/lcdui/Image;)V", Self::init_for_image, Default::default()),
                JavaMethodProto::new("setColor", "(I)V", Self::set_color, Default::default()),
                JavaMethodProto::new("setColor", "(III)V", Self::set_color_rgb, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("getRedComponent", "()I", Self::get_red_component, Default::default()),
                JavaMethodProto::new("getGreenComponent", "()I", Self::get_green_component, Default::default()),
                JavaMethodProto::new("getBlueComponent", "()I", Self::get_blue_component, Default::default()),
                JavaMethodProto::new("setGrayScale", "(I)V", Self::set_gray_scale, Default::default()),
                JavaMethodProto::new("getGrayScale", "()I", Self::get_gray_scale, Default::default()),
                JavaMethodProto::new("setFont", "(Ljavax/microedition/lcdui/Font;)V", Self::set_font, Default::default()),
                JavaMethodProto::new("getFont", "()Ljavax/microedition/lcdui/Font;", Self::get_font, Default::default()),
                JavaMethodProto::new("translate", "(II)V", Self::translate, Default::default()),
                JavaMethodProto::new("getTranslateX", "()I", Self::get_translate_x, Default::default()),
                JavaMethodProto::new("getTranslateY", "()I", Self::get_translate_y, Default::default()),
                JavaMethodProto::new("setClip", "(IIII)V", Self::set_clip, Default::default()),
                JavaMethodProto::new("clipRect", "(IIII)V", Self::clip_rect, Default::default()),
                JavaMethodProto::new("getClipX", "()I", Self::get_clip_x, Default::default()),
                JavaMethodProto::new("getClipY", "()I", Self::get_clip_y, Default::default()),
                JavaMethodProto::new("getClipWidth", "()I", Self::get_clip_width, Default::default()),
                JavaMethodProto::new("getClipHeight", "()I", Self::get_clip_height, Default::default()),
                JavaMethodProto::new("setStrokeStyle", "(I)V", Self::set_stroke_style, Default::default()),
                JavaMethodProto::new("getStrokeStyle", "()I", Self::get_stroke_style, Default::default()),
                JavaMethodProto::new("drawString", "(Ljava/lang/String;III)V", Self::draw_string, Default::default()),
                JavaMethodProto::new("drawSubstring", "(Ljava/lang/String;IIIII)V", Self::draw_substring, Default::default()),
                JavaMethodProto::new("drawChar", "(CIII)V", Self::draw_char, Default::default()),
                JavaMethodProto::new("drawChars", "([CIIIII)V", Self::draw_chars, Default::default()),
                JavaMethodProto::new(
                    "drawImage",
                    "(Ljavax/microedition/lcdui/Image;III)V",
                    Self::draw_image,
                    Default::default(),
                ),
                JavaMethodProto::new("drawLine", "(IIII)V", Self::draw_line, Default::default()),
                JavaMethodProto::new("drawRect", "(IIII)V", Self::draw_rect, Default::default()),
                JavaMethodProto::new("fillRect", "(IIII)V", Self::fill_rect, Default::default()),
                JavaMethodProto::new("drawRoundRect", "(IIIIII)V", Self::draw_round_rect, Default::default()),
                JavaMethodProto::new("fillRoundRect", "(IIIIII)V", Self::fill_round_rect, Default::default()),
                JavaMethodProto::new("drawArc", "(IIIIII)V", Self::draw_arc, Default::default()),
                JavaMethodProto::new("fillArc", "(IIIIII)V", Self::fill_arc, Default::default()),
                JavaMethodProto::new("fillTriangle", "(IIIIII)V", Self::fill_triangle, Default::default()),
                JavaMethodProto::new(
                    "drawRegion",
                    "(Ljavax/microedition/lcdui/Image;IIIIIIII)V",
                    Self::draw_region,
                    Default::default(),
                ),
                JavaMethodProto::new("drawRGB", "([IIIIIIIZ)V", Self::draw_rgb, Default::default()),
                JavaMethodProto::new("copyArea", "(IIIIIII)V", Self::copy_area, Default::default()),
                JavaMethodProto::new("getDisplayColor", "(I)I", Self::get_display_color, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("color", "I", Default::default()),
                JavaFieldProto::new("font", "Ljavax/microedition/lcdui/Font;", Default::default()),
                JavaFieldProto::new("clipX", "I", Default::default()),
                JavaFieldProto::new("clipY", "I", Default::default()),
                JavaFieldProto::new("clipW", "I", Default::default()),
                JavaFieldProto::new("clipH", "I", Default::default()),
                JavaFieldProto::new("translateX", "I", Default::default()),
                JavaFieldProto::new("translateY", "I", Default::default()),
                JavaFieldProto::new("strokeStyle", "I", Default::default()),
                JavaFieldProto::new(TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC, Default::default()),
                JavaFieldProto::new("HCENTER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("VCENTER", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LEFT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RIGHT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TOP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BOTTOM", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BASELINE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SOLID", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DOTTED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Graphics";
        jvm.put_static_field(class, "HCENTER", "I", 1).await?;
        jvm.put_static_field(class, "VCENTER", "I", 2).await?;
        jvm.put_static_field(class, "LEFT", "I", 4).await?;
        jvm.put_static_field(class, "RIGHT", "I", 8).await?;
        jvm.put_static_field(class, "TOP", "I", 16).await?;
        jvm.put_static_field(class, "BOTTOM", "I", 32).await?;
        jvm.put_static_field(class, "BASELINE", "I", 64).await?;
        jvm.put_static_field(class, "SOLID", "I", 0).await?;
        jvm.put_static_field(class, "DOTTED", "I", 1).await
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Graphics::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::init_state(jvm, context, &mut this, None.into()).await
    }

    async fn init_for_image(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Image>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Graphics::<init>({this:?}, {target:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        Self::init_state(jvm, context, &mut this, target).await
    }

    async fn init_state(jvm: &Jvm, context: &mut RuntimeContext, this: &mut ClassInstanceRef<Self>, target: ClassInstanceRef<Image>) -> Result<()> {
        let (width, height) = if target.is_null() {
            (context.screen_width(), context.screen_height())
        } else {
            (jvm.get_field(&target, "width", "I").await?, jvm.get_field(&target, "height", "I").await?)
        };

        jvm.put_field(this, "color", "I", 0).await?;
        jvm.put_field(this, "clipX", "I", 0).await?;
        jvm.put_field(this, "clipY", "I", 0).await?;
        jvm.put_field(this, "clipW", "I", width).await?;
        jvm.put_field(this, "clipH", "I", height).await?;
        jvm.put_field(this, "translateX", "I", 0).await?;
        jvm.put_field(this, "translateY", "I", 0).await?;
        let font = Self::default_font(jvm).await?;
        jvm.put_field(this, "strokeStyle", "I", 0).await?;
        jvm.put_field(this, "font", "Ljavax/microedition/lcdui/Font;", font).await?;
        jvm.put_field(this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC, target).await?;
        this.clear_native_scratch();
        Ok(())
    }

    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, rgb: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::setColor({this:?}, {rgb:#x})");

        let rgb = rgb & 0x00ff_ffff;
        if !this.put_named_field("color", "I", JavaValue::Int(rgb)) {
            jvm.put_field(&mut this, "color", "I", rgb).await?;
        }
        this.clear_native_scratch();

        Ok(())
    }

    async fn set_color_rgb(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, red: i32, green: i32, blue: i32) -> Result<()> {
        let rgb = ((red & 0xff) << 16) | ((green & 0xff) << 8) | (blue & 0xff);
        let _: () = jvm.invoke_virtual(&this, "setColor", "(I)V", (rgb,)).await?;

        Ok(())
    }

    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        if this.get_named_field("color", "I").is_some() {
            return Ok(super::raster::named_i32(&this, "color"));
        }
        jvm.get_field(&this, "color", "I").await
    }

    async fn get_red_component(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let color: i32 = jvm.get_field(&this, "color", "I").await?;
        Ok((color >> 16) & 0xff)
    }

    async fn get_green_component(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let color: i32 = jvm.get_field(&this, "color", "I").await?;
        Ok((color >> 8) & 0xff)
    }

    async fn get_blue_component(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let color: i32 = jvm.get_field(&this, "color", "I").await?;
        Ok(color & 0xff)
    }

    async fn set_gray_scale(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.clamp(0, 255);
        let rgb = (value << 16) | (value << 8) | value;
        let _: () = jvm.invoke_virtual(&this, "setColor", "(I)V", (rgb,)).await?;
        Ok(())
    }

    async fn get_gray_scale(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let color: i32 = jvm.get_field(&this, "color", "I").await?;
        let red = (color >> 16) & 0xff;
        let green = (color >> 8) & 0xff;
        let blue = color & 0xff;
        Ok((red * 30 + green * 59 + blue * 11 + 50) / 100)
    }

    async fn set_font(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, font: ClassInstanceRef<Font>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::setFont({this:?}, {font:?})");

        let font = if font.is_null() { Self::default_font(jvm).await? } else { font };
        jvm.put_field(&mut this, "font", "Ljavax/microedition/lcdui/Font;", font).await?;

        Ok(())
    }

    async fn get_font(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Font>> {
        Self::font_for_graphics(jvm, &this).await
    }

    async fn default_font(jvm: &Jvm) -> Result<ClassInstanceRef<Font>> {
        Ok(jvm.new_class("javax/microedition/lcdui/Font", "(III)V", (0, 0, 0)).await?.into())
    }

    async fn font_for_graphics(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Font>> {
        let font: ClassInstanceRef<Font> = jvm.get_field(this, "font", "Ljavax/microedition/lcdui/Font;").await?;
        if font.is_null() {
            return Self::default_font(jvm).await;
        }
        Ok(font)
    }

    pub(super) async fn text_metrics(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<TextMetrics> {
        let font = Self::font_for_graphics(jvm, this).await?;
        let size: i32 = jvm.get_field(&font, "size", "I").await?;
        Ok(TextMetrics {
            advance: Font::advance_for_size(size),
            height: Font::height_for_size(size),
            baseline: Font::baseline_for_size(size),
            scale: Font::pixel_scale_for_size(size),
        })
    }

    async fn translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        let translate_x: i32 = jvm.get_field(&this, "translateX", "I").await?;
        let translate_y: i32 = jvm.get_field(&this, "translateY", "I").await?;
        jvm.put_field(&mut this, "translateX", "I", translate_x + x).await?;
        jvm.put_field(&mut this, "translateY", "I", translate_y + y).await?;
        this.clear_native_scratch();
        Ok(())
    }

    async fn get_translate_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "translateX", "I").await
    }

    async fn get_translate_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "translateY", "I").await
    }

    async fn set_clip(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::setClip({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let (translate_x, translate_y) = Self::translation(&this);
        jvm.put_field(&mut this, "clipX", "I", x + translate_x).await?;
        jvm.put_field(&mut this, "clipY", "I", y + translate_y).await?;
        jvm.put_field(&mut this, "clipW", "I", width.max(0)).await?;
        jvm.put_field(&mut this, "clipH", "I", height.max(0)).await?;
        this.clear_native_scratch();

        Ok(())
    }

    async fn clip_rect(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::clipRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let (translate_x, translate_y) = Self::translation(&this);
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(&this);
        let x = x + translate_x;
        let y = y + translate_y;
        let x0 = clip_x.max(x);
        let y0 = clip_y.max(y);
        let x1 = (clip_x + clip_w).min(x + width.max(0));
        let y1 = (clip_y + clip_h).min(y + height.max(0));
        jvm.put_field(&mut this, "clipX", "I", x0).await?;
        jvm.put_field(&mut this, "clipY", "I", y0).await?;
        jvm.put_field(&mut this, "clipW", "I", (x1 - x0).max(0)).await?;
        jvm.put_field(&mut this, "clipH", "I", (y1 - y0).max(0)).await?;
        this.clear_native_scratch();
        Ok(())
    }

    async fn get_clip_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let clip_x: i32 = jvm.get_field(&this, "clipX", "I").await?;
        let translate_x: i32 = jvm.get_field(&this, "translateX", "I").await?;
        Ok(clip_x - translate_x)
    }

    async fn get_clip_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let clip_y: i32 = jvm.get_field(&this, "clipY", "I").await?;
        let translate_y: i32 = jvm.get_field(&this, "translateY", "I").await?;
        Ok(clip_y - translate_y)
    }

    async fn get_clip_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "clipW", "I").await
    }

    async fn get_clip_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "clipH", "I").await
    }

    async fn set_stroke_style(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, style: i32) -> Result<()> {
        let style = if style == 1 { 1 } else { 0 };
        jvm.put_field(&mut this, "strokeStyle", "I", style).await
    }

    async fn get_stroke_style(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "strokeStyle", "I").await
    }

    async fn get_display_color(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, color: i32) -> Result<i32> {
        Ok(color & 0x00ff_ffff)
    }

    async fn copy_area(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x_src: i32,
        y_src: i32,
        width: i32,
        height: i32,
        x_dest: i32,
        y_dest: i32,
        _anchor: i32,
    ) -> Result<()> {
        if width <= 0 || height <= 0 {
            return Ok(());
        }
        let target: ClassInstanceRef<Image> = jvm.get_field(&this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC).await?;
        if target.is_null() {
            return Ok(());
        }
        let rgb = jvm.instantiate_array("I", (width * height) as usize).await?;
        let _: () = jvm
            .invoke_virtual(&target, "getRGB", "([IIIIIII)V", (rgb.clone(), 0, width, x_src, y_src, width, height))
            .await?;
        Self::draw_rgb(jvm, context, this, rgb.into(), 0, width, x_dest, y_dest, width, height, false).await
    }
}
