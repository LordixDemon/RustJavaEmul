use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        com::mascotcapsule::micro3d::v3::{invalidate_gpu_image, publish_gpu_image_to_screen},
        java::lang::String,
        javax::microedition::lcdui::{Font, Image},
    },
};

// class javax.microedition.lcdui.Graphics
pub struct Graphics;

const TARGET_IMAGE_FIELD: &str = "targetImage";
const TARGET_IMAGE_DESC: &str = "Ljavax/microedition/lcdui/Image;";

#[derive(Clone, Copy)]
struct TextMetrics {
    advance: i32,
    height: i32,
    baseline: i32,
    scale: i32,
}

impl Graphics {
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
        jvm.put_field(this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC, target).await
    }

    async fn set_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, rgb: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::setColor({this:?}, {rgb:#x})");

        jvm.put_field(&mut this, "color", "I", rgb & 0x00ff_ffff).await?;

        Ok(())
    }

    async fn set_color_rgb(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, red: i32, green: i32, blue: i32) -> Result<()> {
        let rgb = ((red & 0xff) << 16) | ((green & 0xff) << 8) | (blue & 0xff);
        let _: () = jvm.invoke_virtual(&this, "setColor", "(I)V", (rgb,)).await?;

        Ok(())
    }

    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
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

    async fn text_metrics(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<TextMetrics> {
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
        jvm.put_field(&mut this, "translateY", "I", translate_y + y).await
    }

    async fn get_translate_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "translateX", "I").await
    }

    async fn get_translate_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "translateY", "I").await
    }

    async fn set_clip(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::setClip({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let (translate_x, translate_y) = Self::translation(jvm, &this).await?;
        jvm.put_field(&mut this, "clipX", "I", x + translate_x).await?;
        jvm.put_field(&mut this, "clipY", "I", y + translate_y).await?;
        jvm.put_field(&mut this, "clipW", "I", width.max(0)).await?;
        jvm.put_field(&mut this, "clipH", "I", height.max(0)).await?;

        Ok(())
    }

    async fn clip_rect(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::clipRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let (translate_x, translate_y) = Self::translation(jvm, &this).await?;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(jvm, &this).await?;
        let x = x + translate_x;
        let y = y + translate_y;
        let x0 = clip_x.max(x);
        let y0 = clip_y.max(y);
        let x1 = (clip_x + clip_w).min(x + width.max(0));
        let y1 = (clip_y + clip_h).min(y + height.max(0));
        jvm.put_field(&mut this, "clipX", "I", x0).await?;
        jvm.put_field(&mut this, "clipY", "I", y0).await?;
        jvm.put_field(&mut this, "clipW", "I", (x1 - x0).max(0)).await?;
        jvm.put_field(&mut this, "clipH", "I", (y1 - y0).max(0)).await
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

    async fn draw_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let string = if string.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &string).await?
        };
        tracing::trace!("javax.microedition.lcdui.Graphics::drawString({this:?}, {string:?}, {x:?}, {y:?}, {anchor:?})");

        Self::draw_text(jvm, context, &this, &string, x, y, anchor).await
    }

    async fn draw_substring(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }
        let text = JavaLangString::to_rust_string(jvm, &string).await?;
        let chars: Vec<char> = text.chars().collect();
        let valid =
            offset >= 0 && length >= 0 && (offset as usize) <= chars.len() && (length as usize) <= chars.len().saturating_sub(offset as usize);
        if !valid {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid substring range").await);
        }
        let substring: RustString = chars[offset as usize..offset as usize + length as usize].iter().collect();
        Self::draw_text(jvm, context, &this, &substring, x, y, anchor).await
    }

    async fn draw_char(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        ch: JavaChar,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let text = RustString::from_utf16_lossy(&[ch]);
        Self::draw_text(jvm, context, &this, &text, x, y, anchor).await
    }

    async fn draw_chars(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "chars is null").await);
        }
        let array_length = jvm.array_length(&chars).await?;
        let valid =
            offset >= 0 && length >= 0 && (offset as usize) <= array_length && (length as usize) <= array_length.saturating_sub(offset as usize);
        if !valid {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid char range").await);
        }

        let chars: Vec<JavaChar> = jvm.load_array(&chars, offset as usize, length as usize).await?;
        let text = RustString::from_utf16_lossy(&chars);
        Self::draw_text(jvm, context, &this, &text, x, y, anchor).await
    }

    async fn draw_image(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawImage({this:?}, {image:?}, {x:?}, {y:?}, {anchor:?})");

        if image.is_null() {
            return Ok(());
        }

        let (width, height, pixel_array) = Image::pixels(jvm, &image).await?;
        let (x, y) = anchor_xy(x, y, width, height, anchor);
        if Self::draw_gpu_image_to_screen(jvm, context, &this, &pixel_array, x, y, width, height, 0, 0).await? {
            return Ok(());
        }

        let pixels = raw_i32_array(jvm, &pixel_array, (width * height) as usize).await?;
        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, true).await
    }

    async fn draw_line(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut x1: i32,
        mut y1: i32,
        x2: i32,
        y2: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawLine({this:?}, {x1:?}, {y1:?}, {x2:?}, {y2:?})");

        let color = Self::argb_color(jvm, &this).await?;
        if y1 == y2 {
            let x = x1.min(x2);
            let width = (x2 - x1).abs() + 1;
            let pixels = vec![color; width as usize];
            return Self::draw_pixels(jvm, context, &this, x, y1, width, 1, &pixels, true).await;
        }
        if x1 == x2 {
            let y = y1.min(y2);
            let height = (y2 - y1).abs() + 1;
            let pixels = vec![color; height as usize];
            return Self::draw_pixels(jvm, context, &this, x1, y, 1, height, &pixels, true).await;
        }

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            Self::draw_pixels(jvm, context, &this, x1, y1, 1, 1, &[color], true).await?;
            if x1 == x2 && y1 == y2 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }

        Ok(())
    }

    async fn draw_rect(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        if width < 0 || height < 0 {
            return Ok(());
        }

        let color = Self::argb_color(jvm, &this).await?;
        let horizontal_width = width + 1;
        let horizontal_pixels = vec![color; horizontal_width as usize];
        Self::draw_pixels(jvm, context, &this, x, y, horizontal_width, 1, &horizontal_pixels, true).await?;
        if height > 0 {
            Self::draw_pixels(jvm, context, &this, x, y + height, horizontal_width, 1, &horizontal_pixels, true).await?;
        }

        if height > 1 {
            let vertical_pixels = vec![color; (height - 1) as usize];
            Self::draw_pixels(jvm, context, &this, x, y + 1, 1, height - 1, &vertical_pixels, true).await?;
            if width > 0 {
                Self::draw_pixels(jvm, context, &this, x + width, y + 1, 1, height - 1, &vertical_pixels, true).await?;
            }
        }

        Ok(())
    }

    async fn fill_rect(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillRect({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let color = Self::argb_color(jvm, &this).await?;
        let target: ClassInstanceRef<Image> = jvm.get_field(&this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC).await?;
        if target.is_null() {
            let (translate_x, translate_y) = Self::translation(jvm, &this).await?;
            let x = x + translate_x;
            let y = y + translate_y;
            let (clip_x, clip_y, clip_w, clip_h) = Self::clip(jvm, &this).await?;
            let (draw_x, draw_y, draw_w, draw_h, _, _) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
            if draw_w > 0 && draw_h > 0 {
                tracing::info!(
                    target: "rustjava_render",
                    "graphics.fillRect target=screen rect={}x{}+{}+{} clip={}x{}+{}+{} color={:#08x}",
                    draw_w,
                    draw_h,
                    draw_x,
                    draw_y,
                    clip_w,
                    clip_h,
                    clip_x,
                    clip_y,
                    color & 0x00ff_ffff
                );
                context.screen_fill_rect(draw_x, draw_y, draw_w, draw_h, color);
            }
            return Ok(());
        }

        let pixels = vec![color; (width * height) as usize];
        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, true).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_round_rect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _arc_width: i32,
        _arc_height: i32,
    ) -> Result<()> {
        Self::draw_rect(jvm, context, this, x, y, width, height).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn fill_round_rect(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _arc_width: i32,
        _arc_height: i32,
    ) -> Result<()> {
        Self::fill_rect(jvm, context, this, x, y, width, height).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_arc(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: i32,
        arc_angle: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::drawArc({this:?}, {x:?}, {y:?}, {width:?}, {height:?}, {start_angle:?}, {arc_angle:?})");

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn fill_arc(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        start_angle: i32,
        arc_angle: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillArc({this:?}, {x:?}, {y:?}, {width:?}, {height:?}, {start_angle:?}, {arc_angle:?})");

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let color = Self::argb_color(jvm, &this).await?;
        let pixels = vec![color; (width * height) as usize];
        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, true).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn fill_triangle(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Graphics::fillTriangle({this:?}, {x1:?}, {y1:?}, {x2:?}, {y2:?}, {x3:?}, {y3:?})");

        let min_x = x1.min(x2).min(x3);
        let max_x = x1.max(x2).max(x3);
        let min_y = y1.min(y2).min(y3);
        let max_y = y1.max(y2).max(y3);
        let color = Self::argb_color(jvm, &this).await?;
        let max_width = max_x - min_x + 1;
        if max_width <= 0 {
            return Ok(());
        }
        let row_pixels = vec![color; max_width as usize];

        for y in min_y..=max_y {
            let mut row_start = None;
            let mut row_end = min_x;
            for x in min_x..=max_x {
                let a = edge(x2, y2, x3, y3, x, y);
                let b = edge(x3, y3, x1, y1, x, y);
                let c = edge(x1, y1, x2, y2, x, y);
                if (a >= 0 && b >= 0 && c >= 0) || (a <= 0 && b <= 0 && c <= 0) {
                    row_start.get_or_insert(x);
                    row_end = x;
                }
            }
            if let Some(row_start) = row_start {
                let width = row_end - row_start + 1;
                Self::draw_pixels(jvm, context, &this, row_start, y, width, 1, &row_pixels[..width as usize], true).await?;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_region(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        transform: i32,
        dest_x: i32,
        dest_y: i32,
        anchor: i32,
    ) -> Result<()> {
        tracing::trace!(
            "javax.microedition.lcdui.Graphics::drawRegion({this:?}, {image:?}, {src_x:?}, {src_y:?}, {width:?}, {height:?}, {transform:?}, {dest_x:?}, {dest_y:?}, {anchor:?})"
        );

        if image.is_null() || width <= 0 || height <= 0 {
            return Ok(());
        }

        let (image_width, image_height, pixel_array) = Image::pixels(jvm, &image).await?;
        let source_pixels = raw_i32_array(jvm, &pixel_array, jvm.array_length(&pixel_array).await?).await?;
        let (draw_width, draw_height) = transformed_size(width, height, transform);

        let source_region_valid = src_x >= 0
            && src_y >= 0
            && i64::from(src_x) + i64::from(width) <= i64::from(image_width)
            && i64::from(src_y) + i64::from(height) <= i64::from(image_height);
        if transform == 0 && source_region_valid {
            let (x, y) = anchor_xy(dest_x, dest_y, draw_width, draw_height, anchor);
            if Self::draw_gpu_image_to_screen(jvm, context, &this, &pixel_array, x, y, draw_width, draw_height, src_x, src_y).await? {
                return Ok(());
            }

            return Self::draw_pixels_strided(
                jvm,
                context,
                &this,
                x,
                y,
                draw_width,
                draw_height,
                &source_pixels,
                image_width,
                src_x,
                src_y,
                true,
            )
            .await;
        }

        let mut pixels = vec![0; (draw_width * draw_height) as usize];

        for y in 0..height {
            for x in 0..width {
                let src_index = ((src_y + y) * image_width + src_x + x) as usize;
                if src_index >= source_pixels.len() {
                    continue;
                }
                let (tx, ty) = transform_point(x, y, width, height, transform);
                if tx >= 0 && ty >= 0 && tx < draw_width && ty < draw_height {
                    pixels[(ty * draw_width + tx) as usize] = source_pixels[src_index];
                }
            }
        }

        let (x, y) = anchor_xy(dest_x, dest_y, draw_width, draw_height, anchor);
        Self::draw_pixels(jvm, context, &this, x, y, draw_width, draw_height, &pixels, true).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_rgb(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        rgb_data: ClassInstanceRef<Array<i32>>,
        offset: i32,
        scan_length: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        process_alpha: bool,
    ) -> Result<()> {
        tracing::trace!(
            "javax.microedition.lcdui.Graphics::drawRGB({this:?}, {rgb_data:?}, {offset:?}, {scan_length:?}, {x:?}, {y:?}, {width:?}, {height:?}, {process_alpha:?})"
        );

        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let mut pixels = Vec::with_capacity((width * height) as usize);
        for row in 0..height {
            let row_offset = offset + row * scan_length;
            if row_offset < 0 {
                pixels.extend((0..width).map(|_| 0));
                continue;
            }
            let row_pixels = raw_i32_range(jvm, &rgb_data, row_offset as usize, width as usize).await?;
            pixels.extend(row_pixels);
        }

        Self::draw_pixels(jvm, context, &this, x, y, width, height, &pixels, process_alpha).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_pixels(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        process_alpha: bool,
    ) -> Result<()> {
        Self::draw_pixels_strided(jvm, context, this, x, y, width, height, pixels, width, 0, 0, process_alpha).await
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn draw_pixels_strided(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) -> Result<()> {
        if width <= 0 || height <= 0 || source_width <= 0 || source_x < 0 || source_y < 0 {
            return Ok(());
        }

        let (translate_x, translate_y) = Self::translation(jvm, this).await?;
        let x = x + translate_x;
        let y = y + translate_y;
        let target: ClassInstanceRef<Image> = jvm.get_field(this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC).await?;
        if target.is_null() {
            let (clip_x, clip_y, clip_w, clip_h) = Self::clip(jvm, this).await?;
            let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
            if draw_w <= 0 || draw_h <= 0 {
                return Ok(());
            }

            tracing::info!(
                target: "rustjava_render",
                "graphics.drawPixels target=screen rect={}x{}+{}+{} src={}+{} srcWidth={} alpha={} clip={}x{}+{}+{}",
                draw_w,
                draw_h,
                draw_x,
                draw_y,
                source_x + src_offset_x,
                source_y + src_offset_y,
                source_width,
                process_alpha,
                clip_w,
                clip_h,
                clip_x,
                clip_y
            );
            context.screen_draw_pixels_strided(
                draw_x,
                draw_y,
                draw_w,
                draw_h,
                pixels,
                source_width,
                source_x + src_offset_x,
                source_y + src_offset_y,
                process_alpha,
            );
            return Ok(());
        }

        let (target_width, target_height, mut target_pixels_array) = Image::pixels(jvm, &target).await?;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(jvm, this).await?;
        let clip_x2 = (clip_x + clip_w).min(target_width);
        let clip_y2 = (clip_y + clip_h).min(target_height);
        let clip_x = clip_x.max(0);
        let clip_y = clip_y.max(0);
        let clip_w = (clip_x2 - clip_x).max(0);
        let clip_h = (clip_y2 - clip_y).max(0);
        let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w <= 0 || draw_h <= 0 {
            return Ok(());
        }
        invalidate_gpu_image(&target_pixels_array);

        let draw_area = draw_w as i64 * draw_h as i64;
        let target_area = target_width as i64 * target_height as i64;
        if draw_area * 4 < target_area {
            let draw_w_usize = draw_w as usize;
            for row in 0..draw_h {
                let src_start = ((source_y + src_offset_y + row) * source_width + source_x + src_offset_x) as usize;
                let Some(src_row) = pixels.get(src_start..src_start + draw_w_usize) else {
                    return Ok(());
                };
                let dst_start = ((draw_y + row) * target_width + draw_x) as usize;

                let row_pixels = if process_alpha {
                    if is_opaque_row(src_row) {
                        src_row.to_vec()
                    } else {
                        let mut dst_row = raw_i32_range(jvm, &target_pixels_array, dst_start, draw_w_usize).await?;
                        if dst_row.len() < draw_w_usize {
                            dst_row.resize(draw_w_usize, 0);
                        }
                        for (dst, src) in dst_row.iter_mut().zip(src_row) {
                            *dst = compose(*dst, *src, true);
                        }
                        dst_row
                    }
                } else {
                    src_row.iter().map(|pixel| (0xff00_0000u32 as i32) | (pixel & 0x00ff_ffff)).collect()
                };
                store_raw_i32_range(jvm, &mut target_pixels_array, dst_start, &row_pixels).await?;
            }

            return Ok(());
        }

        let mut target_pixels = raw_i32_array(jvm, &target_pixels_array, (target_width * target_height) as usize).await?;

        for row in 0..draw_h {
            let dst_y = draw_y + row;
            if dst_y < 0 || dst_y >= target_height || dst_y < clip_y || dst_y >= clip_y + clip_h {
                continue;
            }
            let src_row_start = (source_y + src_offset_y + row) * source_width + source_x + src_offset_x;
            for col in 0..draw_w {
                let dst_x = draw_x + col;
                if dst_x < 0 || dst_x >= target_width || dst_x < clip_x || dst_x >= clip_x + clip_w {
                    continue;
                }
                let src = pixels[(src_row_start + col) as usize];
                let dst_index = (dst_y * target_width + dst_x) as usize;
                target_pixels[dst_index] = compose(target_pixels[dst_index], src, process_alpha);
            }
        }

        store_raw_i32_range(jvm, &mut target_pixels_array, 0, &target_pixels).await
    }

    async fn argb_color(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<i32> {
        let rgb: i32 = jvm.get_field(this, "color", "I").await?;
        Ok((0xff00_0000u32 as i32) | (rgb & 0x00ff_ffff))
    }

    async fn draw_text(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        text: &str,
        x: i32,
        y: i32,
        anchor: i32,
    ) -> Result<()> {
        let color = Self::argb_color(jvm, this).await?;
        let metrics = Self::text_metrics(jvm, this).await?;
        let (width, height, pixels) = rasterize_text(text, color, metrics);
        let (x, y) = text_anchor_xy(x, y, width, height, metrics.baseline, anchor);
        Self::draw_pixels(jvm, context, this, x, y, width, height, &pixels, true).await
    }

    async fn translation(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<(i32, i32)> {
        Ok((
            jvm.get_field(this, "translateX", "I").await?,
            jvm.get_field(this, "translateY", "I").await?,
        ))
    }

    async fn clip(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<(i32, i32, i32, i32)> {
        Ok((
            jvm.get_field(this, "clipX", "I").await?,
            jvm.get_field(this, "clipY", "I").await?,
            jvm.get_field(this, "clipW", "I").await?,
            jvm.get_field(this, "clipH", "I").await?,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    async fn draw_gpu_image_to_screen(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &ClassInstanceRef<Self>,
        pixels: &ClassInstanceRef<Array<i32>>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        source_x: i32,
        source_y: i32,
    ) -> Result<bool> {
        let target: ClassInstanceRef<Image> = jvm.get_field(this, TARGET_IMAGE_FIELD, TARGET_IMAGE_DESC).await?;
        if !target.is_null() {
            return Ok(false);
        }

        let (translate_x, translate_y) = Self::translation(jvm, this).await?;
        let x = x + translate_x;
        let y = y + translate_y;
        let (clip_x, clip_y, clip_w, clip_h) = Self::clip(jvm, this).await?;
        let (draw_x, draw_y, draw_w, draw_h, src_offset_x, src_offset_y) = clipped_rect(x, y, width, height, clip_x, clip_y, clip_w, clip_h);
        if draw_w <= 0 || draw_h <= 0 {
            return Ok(false);
        }

        Ok(publish_gpu_image_to_screen(
            pixels,
            context.screen_width(),
            context.screen_height(),
            draw_x,
            draw_y,
            source_x + src_offset_x,
            source_y + src_offset_y,
            draw_w,
            draw_h,
            (clip_x, clip_y, clip_w, clip_h),
        ))
    }
}

async fn raw_i32_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i32>>, count: usize) -> Result<Vec<i32>> {
    raw_i32_range(jvm, array, 0, count).await
}

async fn raw_i32_range(jvm: &Jvm, array: &ClassInstanceRef<Array<i32>>, offset: usize, count: usize) -> Result<Vec<i32>> {
    if count == 0 {
        return Ok(Vec::new());
    }

    let length = jvm.array_length(array).await?;
    if offset >= length {
        return Ok(Vec::new());
    }

    let count = count.min(length - offset);
    let mut values = vec![0; count];
    jvm.array_raw_buffer(array)
        .await?
        .read(offset, bytemuck::cast_slice_mut(values.as_mut_slice()))?;

    #[cfg(target_endian = "big")]
    for value in &mut values {
        *value = i32::from_le(*value);
    }

    Ok(values)
}

async fn store_raw_i32_range(jvm: &Jvm, array: &mut ClassInstanceRef<Array<i32>>, offset: usize, values: &[i32]) -> Result<()> {
    if values.is_empty() {
        return Ok(());
    }

    let length = jvm.array_length(array).await?;
    if offset >= length {
        return Ok(());
    }

    let count = values.len().min(length - offset);
    #[cfg(target_endian = "little")]
    {
        jvm.array_raw_buffer_mut(array)
            .await?
            .write(offset, bytemuck::cast_slice(&values[..count]))
    }

    #[cfg(target_endian = "big")]
    {
        let converted = values[..count].iter().map(|value| value.to_le()).collect::<Vec<_>>();
        jvm.array_raw_buffer_mut(array)
            .await?
            .write(offset, bytemuck::cast_slice(converted.as_slice()))
    }
}

fn is_opaque_row(pixels: &[i32]) -> bool {
    pixels.iter().all(|pixel| ((*pixel as u32) >> 24) == 0xff)
}

fn edge(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i32 {
    (px - ax) * (by - ay) - (py - ay) * (bx - ax)
}

fn anchor_xy(mut x: i32, mut y: i32, width: i32, height: i32, anchor: i32) -> (i32, i32) {
    if anchor & 1 != 0 {
        x -= width / 2;
    } else if anchor & 8 != 0 {
        x -= width;
    }

    if anchor & 2 != 0 {
        y -= height / 2;
    } else if anchor & 32 != 0 {
        y -= height;
    }

    (x, y)
}

fn text_anchor_xy(mut x: i32, mut y: i32, width: i32, height: i32, baseline: i32, anchor: i32) -> (i32, i32) {
    if anchor & 1 != 0 {
        x -= width / 2;
    } else if anchor & 8 != 0 {
        x -= width;
    }

    if anchor & 2 != 0 {
        y -= height / 2;
    } else if anchor & 32 != 0 {
        y -= height;
    } else if anchor & 64 != 0 {
        y -= baseline;
    }

    (x, y)
}

fn rasterize_text(text: &str, color: i32, metrics: TextMetrics) -> (i32, i32, Vec<i32>) {
    let width = (text.chars().count() as i32 * metrics.advance).max(1);
    let height = metrics.height.max(1);
    let scale = metrics.scale.max(1);
    let glyph_height = 7 * scale;
    let glyph_y = (height - glyph_height).max(0) / 2;
    let mut pixels = vec![0; (width * height) as usize];

    for (char_index, ch) in text.chars().enumerate() {
        let glyph = glyph_5x7(ch);
        let base_x = char_index as i32 * metrics.advance;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    let glyph_x = base_x + col * scale;
                    let glyph_y = glyph_y + row as i32 * scale;
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let px = glyph_x + sx;
                            let py = glyph_y + sy;
                            if px >= 0 && px < width && py >= 0 && py < height {
                                pixels[(py * width + px) as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }

    (width, height, pixels)
}

fn glyph_5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0x0e, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'B' => [0x1e, 0x11, 0x11, 0x1e, 0x11, 0x11, 0x1e],
        'C' => [0x0e, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0e],
        'D' => [0x1e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1e],
        'E' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x1f],
        'F' => [0x1f, 0x10, 0x10, 0x1e, 0x10, 0x10, 0x10],
        'G' => [0x0e, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0f],
        'H' => [0x11, 0x11, 0x11, 0x1f, 0x11, 0x11, 0x11],
        'I' => [0x0e, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0e],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x12, 0x12, 0x0c],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1f],
        'M' => [0x11, 0x1b, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x19, 0x15, 0x13, 0x11, 0x11, 0x11],
        'O' => [0x0e, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'P' => [0x1e, 0x11, 0x11, 0x1e, 0x10, 0x10, 0x10],
        'Q' => [0x0e, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0d],
        'R' => [0x1e, 0x11, 0x11, 0x1e, 0x14, 0x12, 0x11],
        'S' => [0x0f, 0x10, 0x10, 0x0e, 0x01, 0x01, 0x1e],
        'T' => [0x1f, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0e],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x0a, 0x0a, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1b, 0x11],
        'X' => [0x11, 0x11, 0x0a, 0x04, 0x0a, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0a, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1f],
        '0' => [0x0e, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0e],
        '1' => [0x04, 0x0c, 0x04, 0x04, 0x04, 0x04, 0x0e],
        '2' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1f],
        '3' => [0x1e, 0x01, 0x01, 0x0e, 0x01, 0x01, 0x1e],
        '4' => [0x02, 0x06, 0x0a, 0x12, 0x1f, 0x02, 0x02],
        '5' => [0x1f, 0x10, 0x10, 0x1e, 0x01, 0x01, 0x1e],
        '6' => [0x0e, 0x10, 0x10, 0x1e, 0x11, 0x11, 0x0e],
        '7' => [0x1f, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0e, 0x11, 0x11, 0x0e, 0x11, 0x11, 0x0e],
        '9' => [0x0e, 0x11, 0x11, 0x0f, 0x01, 0x01, 0x0e],
        ':' => [0x00, 0x04, 0x04, 0x00, 0x04, 0x04, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0c],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x0c, 0x04, 0x08],
        '!' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '?' => [0x0e, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
        '-' => [0x00, 0x00, 0x00, 0x1f, 0x00, 0x00, 0x00],
        '_' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f],
        '/' => [0x01, 0x01, 0x02, 0x04, 0x08, 0x10, 0x10],
        '\\' => [0x10, 0x10, 0x08, 0x04, 0x02, 0x01, 0x01],
        '+' => [0x00, 0x04, 0x04, 0x1f, 0x04, 0x04, 0x00],
        '*' => [0x00, 0x15, 0x0e, 0x1f, 0x0e, 0x15, 0x00],
        '=' => [0x00, 0x00, 0x1f, 0x00, 0x1f, 0x00, 0x00],
        '(' => [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
        ')' => [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
        '[' => [0x0e, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0e],
        ']' => [0x0e, 0x02, 0x02, 0x02, 0x02, 0x02, 0x0e],
        '\'' => [0x04, 0x04, 0x08, 0x00, 0x00, 0x00, 0x00],
        '"' => [0x0a, 0x0a, 0x14, 0x00, 0x00, 0x00, 0x00],
        ' ' => [0; 7],
        _ => [0x1f, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
    }
}

#[allow(clippy::too_many_arguments)]
fn clipped_rect(x: i32, y: i32, width: i32, height: i32, clip_x: i32, clip_y: i32, clip_w: i32, clip_h: i32) -> (i32, i32, i32, i32, i32, i32) {
    let draw_x = x.max(clip_x);
    let draw_y = y.max(clip_y);
    let end_x = (x + width).min(clip_x + clip_w);
    let end_y = (y + height).min(clip_y + clip_h);

    (draw_x, draw_y, end_x - draw_x, end_y - draw_y, draw_x - x, draw_y - y)
}

fn transformed_size(width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        4..=7 => (height, width),
        _ => (width, height),
    }
}

fn transform_point(x: i32, y: i32, width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        1 => (x, height - 1 - y),             // mirror + rot180
        2 => (width - 1 - x, y),              // mirror
        3 => (width - 1 - x, height - 1 - y), // rot180
        4 => (y, x),                          // mirror + rot270
        5 => (height - 1 - y, x),             // rot90
        6 => (y, width - 1 - x),              // rot270
        7 => (height - 1 - y, width - 1 - x), // mirror + rot90
        _ => (x, y),
    }
}

fn compose(dst: i32, src: i32, process_alpha: bool) -> i32 {
    if !process_alpha {
        return (0xff00_0000u32 as i32) | (src & 0x00ff_ffff);
    }

    let alpha = ((src as u32) >> 24) & 0xff;
    if alpha == 0xff {
        return src;
    }
    if alpha == 0 {
        return dst;
    }

    let inv = 255 - alpha;
    let src = src as u32;
    let dst = dst as u32;
    let r = (((src >> 16) & 0xff) * alpha + ((dst >> 16) & 0xff) * inv) / 255;
    let g = (((src >> 8) & 0xff) * alpha + ((dst >> 8) & 0xff) * inv) / 255;
    let b = ((src & 0xff) * alpha + (dst & 0xff) * inv) / 255;

    (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
}
