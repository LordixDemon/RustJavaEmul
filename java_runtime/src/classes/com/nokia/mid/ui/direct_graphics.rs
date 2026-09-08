#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image},
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl DirectGraphics {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/ui/DirectGraphics",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/lcdui/Graphics;)V",
                    Self::init_graphics,
                    Default::default(),
                ),
                JavaMethodProto::new("setARGBColor", "(I)V", Self::set_argb_color, Default::default()),
                JavaMethodProto::new("getAlphaComponent", "()I", Self::get_alpha, Default::default()),
                JavaMethodProto::new("getNativePixelFormat", "()I", Self::get_native_pixel_format, Default::default()),
                JavaMethodProto::new("drawPixels", "([B[BIIIIIIII)V", Self::draw_pixels_bytes, Default::default()),
                JavaMethodProto::new("drawPixels", "([S[BIIIIIIII)V", Self::draw_pixels_shorts, Default::default()),
                JavaMethodProto::new("drawPixels", "([I[BIIIIIIII)V", Self::draw_pixels_ints, Default::default()),
                JavaMethodProto::new("getPixels", "([B[BIIIIIII)V", Self::get_pixels_bytes, Default::default()),
                JavaMethodProto::new("getPixels", "([S[BIIIIIII)V", Self::get_pixels_shorts, Default::default()),
                JavaMethodProto::new("getPixels", "([I[BIIIIIII)V", Self::get_pixels_ints, Default::default()),
                JavaMethodProto::new("getPixels", "([IIIIIIII)V", Self::get_pixels_ints_no_mask, Default::default()),
                JavaMethodProto::new("getPixels", "([SIIIIIII)V", Self::get_pixels_shorts_no_mask_7, Default::default()),
                JavaMethodProto::new("getPixels", "([SIIIIIIII)V", Self::get_pixels_shorts_no_mask, Default::default()),
                JavaMethodProto::new("drawPolygon", "([I[III)V", Self::draw_polygon, Default::default()),
                JavaMethodProto::new("fillPolygon", "([I[III)V", Self::fill_polygon, Default::default()),
                JavaMethodProto::new("drawTriangle", "(IIIIII)V", Self::draw_triangle, Default::default()),
                JavaMethodProto::new("fillTriangle", "(IIIIII)V", Self::fill_triangle, Default::default()),
                JavaMethodProto::new(
                    "drawImage",
                    "(Ljavax/microedition/lcdui/Image;IIII)V",
                    Self::draw_image,
                    Default::default(),
                ),
                JavaMethodProto::new("drawPixels", "([IZIIIIIIII)V", Self::draw_pixels_ints_alpha, Default::default()),
                JavaMethodProto::new("drawPixels", "([SZIIIIIIII)V", Self::draw_pixels_shorts_alpha, Default::default()),
                JavaMethodProto::new("fillPolygon", "([II[IIII)V", Self::fill_polygon_offset, Default::default()),
                JavaMethodProto::new("fillTriangle", "(IIIIIII)V", Self::fill_triangle_color, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("argbColor", "I", Default::default()),
                JavaFieldProto::new("graphics", "Ljavax/microedition/lcdui/Graphics;", Default::default()),
                JavaFieldProto::new("TYPE_USHORT_4444_ARGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_USHORT_444_RGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_USHORT_555_RGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_USHORT_1555_ARGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_USHORT_565_RGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_INT_888_RGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_INT_8888_ARGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_1_GRAY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_1_GRAY_VERTICAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_2_GRAY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_4_GRAY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_8_GRAY", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TYPE_BYTE_332_RGB", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FLIP_HORIZONTAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FLIP_VERTICAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ROTATE_90", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ROTATE_180", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ROTATE_270", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "com/nokia/mid/ui/DirectGraphics";
        jvm.put_static_field(class, "TYPE_USHORT_4444_ARGB", "I", 4444).await?;
        jvm.put_static_field(class, "TYPE_USHORT_444_RGB", "I", 444).await?;
        jvm.put_static_field(class, "TYPE_USHORT_555_RGB", "I", 555).await?;
        jvm.put_static_field(class, "TYPE_USHORT_1555_ARGB", "I", 1555).await?;
        jvm.put_static_field(class, "TYPE_USHORT_565_RGB", "I", 565).await?;
        jvm.put_static_field(class, "TYPE_INT_888_RGB", "I", 888).await?;
        jvm.put_static_field(class, "TYPE_INT_8888_ARGB", "I", 8888).await?;
        jvm.put_static_field(class, "TYPE_BYTE_1_GRAY", "I", 1).await?;
        jvm.put_static_field(class, "TYPE_BYTE_1_GRAY_VERTICAL", "I", -1).await?;
        jvm.put_static_field(class, "TYPE_BYTE_2_GRAY", "I", 2).await?;
        jvm.put_static_field(class, "TYPE_BYTE_4_GRAY", "I", 4).await?;
        jvm.put_static_field(class, "TYPE_BYTE_8_GRAY", "I", 8).await?;
        jvm.put_static_field(class, "TYPE_BYTE_332_RGB", "I", 332).await?;
        jvm.put_static_field(class, "FLIP_HORIZONTAL", "I", 0x2000).await?;
        jvm.put_static_field(class, "FLIP_VERTICAL", "I", 0x4000).await?;
        jvm.put_static_field(class, "ROTATE_90", "I", 1).await?;
        jvm.put_static_field(class, "ROTATE_180", "I", 2).await?;
        jvm.put_static_field(class, "ROTATE_270", "I", 3).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    pub(super) async fn init_graphics(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        graphics: ClassInstanceRef<Graphics>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "graphics", "Ljavax/microedition/lcdui/Graphics;", graphics)
            .await
    }

    pub(super) async fn set_argb_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, argb: i32) -> Result<()> {
        jvm.put_field(&mut this, "argbColor", "I", argb).await
    }

    pub(super) async fn get_alpha(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let argb: i32 = jvm.get_field(&this, "argbColor", "I").await?;
        Ok(((argb as u32) >> 24) as i32)
    }

    pub(super) async fn get_native_pixel_format(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(8888)
    }

    pub(super) async fn draw_pixels_ints(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        pixels: ClassInstanceRef<Array<i32>>,
        _transparency: ClassInstanceRef<Array<i8>>,
        offset: i32,
        scanlength: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _manipulation: i32,
        _format: i32,
    ) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "graphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() || pixels.is_null() || width <= 0 || height <= 0 {
            return Ok(());
        }
        jvm.invoke_virtual(
            &graphics,
            "drawRGB",
            "([IIIIIIIZ)V",
            (pixels, offset, scanlength, x, y, width, height, true),
        )
        .await
    }

    pub(super) async fn draw_pixels_shorts(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        pixels: ClassInstanceRef<Array<i16>>,
        _transparency: ClassInstanceRef<Array<i8>>,
        offset: i32,
        scanlength: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _manipulation: i32,
        format: i32,
    ) -> Result<()> {
        if pixels.is_null() || width <= 0 || height <= 0 {
            return Ok(());
        }
        let count = (height * scanlength.max(width)).max(0) as usize;
        let src: alloc::vec::Vec<i16> = jvm
            .load_array(&pixels, offset.max(0) as usize, count.min(jvm.array_length(&pixels).await?))
            .await?;
        let mut argb = alloc::vec::Vec::with_capacity(src.len());
        for value in src {
            argb.push(short_to_argb(value as u16, format));
        }
        let mut array = jvm.instantiate_array("I", argb.len()).await?;
        jvm.store_array(&mut array, 0, argb).await?;
        Self::draw_pixels_ints(
            jvm,
            context,
            this,
            array.into(),
            ClassInstanceRef::new(None),
            0,
            scanlength,
            x,
            y,
            width,
            height,
            0,
            8888,
        )
        .await
    }

    pub(super) async fn draw_pixels_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        pixels: ClassInstanceRef<Array<i8>>,
        _transparency: ClassInstanceRef<Array<i8>>,
        offset: i32,
        scanlength: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        _manipulation: i32,
        _format: i32,
    ) -> Result<()> {
        if pixels.is_null() || width <= 0 || height <= 0 {
            return Ok(());
        }
        let count = (height * scanlength.max(width)).max(0) as usize;
        let src: alloc::vec::Vec<i8> = jvm
            .load_array(&pixels, offset.max(0) as usize, count.min(jvm.array_length(&pixels).await?))
            .await?;
        let argb: alloc::vec::Vec<i32> = src.into_iter().map(|b| 0xff00_0000u32 as i32 | (b as u8 as i32) * 0x010101).collect();
        let mut array = jvm.instantiate_array("I", argb.len()).await?;
        jvm.store_array(&mut array, 0, argb).await?;
        Self::draw_pixels_ints(
            jvm,
            context,
            this,
            array.into(),
            ClassInstanceRef::new(None),
            0,
            scanlength,
            x,
            y,
            width,
            height,
            0,
            8888,
        )
        .await
    }

    pub(super) async fn get_pixels_ints(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i32>>,
        _t: ClassInstanceRef<Array<i8>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _f: i32,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_pixels_ints_no_mask(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i32>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _f: i32,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_pixels_shorts_no_mask_7(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i16>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_pixels_shorts_no_mask(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i16>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _f: i32,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_pixels_shorts(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i16>>,
        _t: ClassInstanceRef<Array<i8>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _f: i32,
    ) -> Result<()> {
        Ok(())
    }
    pub(super) async fn get_pixels_bytes(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _p: ClassInstanceRef<Array<i8>>,
        _t: ClassInstanceRef<Array<i8>>,
        _o: i32,
        _s: i32,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
        _f: i32,
    ) -> Result<()> {
        Ok(())
    }

    pub(super) async fn draw_polygon(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x_points: ClassInstanceRef<Array<i32>>,
        y_points: ClassInstanceRef<Array<i32>>,
        n: i32,
        _argb: i32,
    ) -> Result<()> {
        Self::poly(jvm, context, this, x_points, y_points, n, false).await
    }
    pub(super) async fn fill_polygon(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x_points: ClassInstanceRef<Array<i32>>,
        y_points: ClassInstanceRef<Array<i32>>,
        n: i32,
        _argb: i32,
    ) -> Result<()> {
        Self::poly(jvm, context, this, x_points, y_points, n, true).await
    }

    pub(super) async fn poly(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x_points: ClassInstanceRef<Array<i32>>,
        y_points: ClassInstanceRef<Array<i32>>,
        n: i32,
        fill: bool,
    ) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "graphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() || x_points.is_null() || y_points.is_null() || n < 2 {
            return Ok(());
        }
        let xs: alloc::vec::Vec<i32> = jvm
            .load_array(&x_points, 0, n.min(jvm.array_length(&x_points).await? as i32) as usize)
            .await?;
        let ys: alloc::vec::Vec<i32> = jvm
            .load_array(&y_points, 0, n.min(jvm.array_length(&y_points).await? as i32) as usize)
            .await?;
        for i in 0..xs.len().saturating_sub(1) {
            let _: () = if fill {
                jvm.invoke_virtual(
                    &graphics,
                    "fillTriangle",
                    "(IIIIII)V",
                    (xs[0], ys[0], xs[i], ys[i], xs[i + 1], ys.get(i + 1).copied().unwrap_or(ys[0])),
                )
                .await?
            } else {
                jvm.invoke_virtual(&graphics, "drawLine", "(IIII)V", (xs[i], ys[i], xs[i + 1], ys[i + 1]))
                    .await?
            };
        }
        Ok(())
    }

    pub(super) async fn draw_triangle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    ) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "graphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() {
            return Ok(());
        }
        let _: () = jvm.invoke_virtual(&graphics, "drawLine", "(IIII)V", (x1, y1, x2, y2)).await?;
        let _: () = jvm.invoke_virtual(&graphics, "drawLine", "(IIII)V", (x2, y2, x3, y3)).await?;
        jvm.invoke_virtual(&graphics, "drawLine", "(IIII)V", (x3, y3, x1, y1)).await
    }

    pub(super) async fn fill_triangle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
    ) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "graphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() {
            return Ok(());
        }
        jvm.invoke_virtual(&graphics, "fillTriangle", "(IIIIII)V", (x1, y1, x2, y2, x3, y3)).await
    }

    pub(super) async fn draw_image(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        anchor: i32,
        _manipulation: i32,
    ) -> Result<()> {
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(&this, "graphics", "Ljavax/microedition/lcdui/Graphics;").await?;
        if graphics.is_null() {
            return Ok(());
        }
        jvm.invoke_virtual(&graphics, "drawImage", "(Ljavax/microedition/lcdui/Image;III)V", (image, x, y, anchor))
            .await
    }

    pub(super) async fn draw_pixels_ints_alpha(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        pixels: ClassInstanceRef<Array<i32>>,
        _transparency: bool,
        offset: i32,
        scanlength: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        manipulation: i32,
        format: i32,
    ) -> Result<()> {
        Self::draw_pixels_ints(
            jvm,
            context,
            this,
            pixels,
            ClassInstanceRef::new(None),
            offset,
            scanlength,
            x,
            y,
            width,
            height,
            manipulation,
            format,
        )
        .await
    }

    pub(super) async fn draw_pixels_shorts_alpha(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        pixels: ClassInstanceRef<Array<i16>>,
        _transparency: bool,
        offset: i32,
        scanlength: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        manipulation: i32,
        format: i32,
    ) -> Result<()> {
        Self::draw_pixels_shorts(
            jvm,
            context,
            this,
            pixels,
            ClassInstanceRef::new(None),
            offset,
            scanlength,
            x,
            y,
            width,
            height,
            manipulation,
            format,
        )
        .await
    }

    pub(super) async fn fill_polygon_offset(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x_points: ClassInstanceRef<Array<i32>>,
        x_offset: i32,
        y_points: ClassInstanceRef<Array<i32>>,
        y_offset: i32,
        n: i32,
        argb: i32,
    ) -> Result<()> {
        if x_points.is_null() || y_points.is_null() || n <= 0 {
            return Ok(());
        }
        let xs: alloc::vec::Vec<i32> = jvm.load_array(&x_points, x_offset.max(0) as usize, n as usize).await?;
        let ys: alloc::vec::Vec<i32> = jvm.load_array(&y_points, y_offset.max(0) as usize, n as usize).await?;
        let mut xa = jvm.instantiate_array("I", n as usize).await?;
        let mut ya = jvm.instantiate_array("I", n as usize).await?;
        jvm.store_array(&mut xa, 0, xs).await?;
        jvm.store_array(&mut ya, 0, ys).await?;
        Self::fill_polygon(jvm, context, this, xa.into(), ya.into(), n, argb).await
    }

    pub(super) async fn fill_triangle_color(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        argb: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_virtual(&this, "setARGBColor", "(I)V", (argb,)).await?;
        Self::fill_triangle(jvm, context, this, x1, y1, x2, y2, x3, y3).await
    }
}

fn short_to_argb(value: u16, format: i32) -> i32 {
    match format {
        565 => {
            let r = ((value >> 11) & 0x1f) * 255 / 31;
            let g = ((value >> 5) & 0x3f) * 255 / 63;
            let b = (value & 0x1f) * 255 / 31;
            0xff00_0000u32 as i32 | ((r as i32) << 16) | ((g as i32) << 8) | b as i32
        }
        4444 => {
            let a = ((value >> 12) & 0xf) * 255 / 15;
            let r = ((value >> 8) & 0xf) * 255 / 15;
            let g = ((value >> 4) & 0xf) * 255 / 15;
            let b = (value & 0xf) * 255 / 15;
            ((a as i32) << 24) | ((r as i32) << 16) | ((g as i32) << 8) | b as i32
        }
        _ => 0xff00_0000u32 as i32 | (value as i32),
    }
}
