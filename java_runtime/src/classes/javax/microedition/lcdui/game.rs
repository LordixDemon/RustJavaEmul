use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        com::mascotcapsule::micro3d::v3::publish_gpu_image_to_screen,
        javax::microedition::lcdui::{Graphics, Image},
    },
};

// class javax.microedition.lcdui.game.GameCanvas
pub struct GameCanvas;
pub struct Sprite;

const BACK_BUFFER_FIELD: &str = "backBuffer";
const BACK_BUFFER_DESC: &str = "Ljavax/microedition/lcdui/Image;";
const BACK_GRAPHICS_FIELD: &str = "backGraphics";
const BACK_GRAPHICS_DESC: &str = "Ljavax/microedition/lcdui/Graphics;";

impl GameCanvas {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/GameCanvas",
            parent_class: Some("javax/microedition/lcdui/Canvas"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Z)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getGraphics",
                    "()Ljavax/microedition/lcdui/Graphics;",
                    Self::get_graphics,
                    Default::default(),
                ),
                JavaMethodProto::new("flushGraphics", "()V", Self::flush_graphics, Default::default()),
                JavaMethodProto::new("flushGraphics", "(IIII)V", Self::flush_graphics_region, Default::default()),
                JavaMethodProto::new("getKeyStates", "()I", Self::get_key_states, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("suppressKeyEvents", "Z", Default::default()),
                JavaFieldProto::new(BACK_BUFFER_FIELD, BACK_BUFFER_DESC, Default::default()),
                JavaFieldProto::new(BACK_GRAPHICS_FIELD, BACK_GRAPHICS_DESC, Default::default()),
                JavaFieldProto::new("UP_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LEFT_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RIGHT_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DOWN_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FIRE_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_A_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_B_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_C_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_D_PRESSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/game/GameCanvas";
        jvm.put_static_field(class, "UP_PRESSED", "I", 0x0002).await?;
        jvm.put_static_field(class, "LEFT_PRESSED", "I", 0x0004).await?;
        jvm.put_static_field(class, "RIGHT_PRESSED", "I", 0x0020).await?;
        jvm.put_static_field(class, "DOWN_PRESSED", "I", 0x0040).await?;
        jvm.put_static_field(class, "FIRE_PRESSED", "I", 0x0100).await?;
        jvm.put_static_field(class, "GAME_A_PRESSED", "I", 0x0200).await?;
        jvm.put_static_field(class, "GAME_B_PRESSED", "I", 0x0400).await?;
        jvm.put_static_field(class, "GAME_C_PRESSED", "I", 0x0800).await?;
        jvm.put_static_field(class, "GAME_D_PRESSED", "I", 0x1000).await
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, suppress_key_events: bool) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.game.GameCanvas::<init>({this:?}, {suppress_key_events:?})");
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.init class={} suppressKeyEvents={} screen={}x{}",
            this.class_definition().name(),
            suppress_key_events,
            context.screen_width(),
            context.screen_height()
        );

        let _: () = jvm.invoke_special(&this, "javax/microedition/lcdui/Canvas", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "suppressKeyEvents", "Z", suppress_key_events).await?;
        jvm.put_field(&mut this, BACK_BUFFER_FIELD, BACK_BUFFER_DESC, ClassInstanceRef::<Image>::new(None))
            .await?;
        jvm.put_field(
            &mut this,
            BACK_GRAPHICS_FIELD,
            BACK_GRAPHICS_DESC,
            ClassInstanceRef::<Graphics>::new(None),
        )
        .await?;
        Self::ensure_back_buffer(jvm, context, &mut this).await?;

        Ok(())
    }

    async fn get_graphics(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Graphics>> {
        tracing::trace!("javax.microedition.lcdui.game.GameCanvas::getGraphics({this:?})");

        let (buffer, _) = Self::ensure_back_buffer(jvm, context, &mut this).await?;
        let (buffer_width, buffer_height, _) = Image::pixels(jvm, &buffer).await?;
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.getGraphics class={} buffer={}x{}",
            this.class_definition().name(),
            buffer_width,
            buffer_height
        );

        let graphics = Self::new_back_graphics(jvm, &buffer).await?;
        jvm.put_field(&mut this, BACK_GRAPHICS_FIELD, BACK_GRAPHICS_DESC, graphics.clone())
            .await?;

        Ok(graphics)
    }

    async fn flush_graphics(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.game.GameCanvas::flushGraphics({this:?})");

        let is_current = Self::is_current_canvas(context, &this);
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.flush class={} current={} region=full",
            this.class_definition().name(),
            is_current
        );
        if !is_current {
            return Ok(());
        }

        let (buffer, _) = Self::ensure_back_buffer(jvm, context, &mut this).await?;
        let (width, height, _) = Image::pixels(jvm, &buffer).await?;
        Self::flush_back_buffer_region(jvm, context, &buffer, 0, 0, width, height).await
    }

    async fn flush_graphics_region(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.game.GameCanvas::flushGraphics({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let is_current = Self::is_current_canvas(context, &this);
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.flush class={} current={} region={}x{}+{}+{}",
            this.class_definition().name(),
            is_current,
            width,
            height,
            x,
            y
        );
        if !is_current {
            return Ok(());
        }

        let (buffer, _) = Self::ensure_back_buffer(jvm, context, &mut this).await?;
        Self::flush_back_buffer_region(jvm, context, &buffer, x, y, width, height).await
    }

    async fn get_key_states(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.game.GameCanvas::getKeyStates({this:?})");

        Ok(context.game_key_states())
    }

    fn is_current_canvas(context: &RuntimeContext, this: &ClassInstanceRef<Self>) -> bool {
        this.instance
            .as_deref()
            .is_some_and(|displayable| context.is_current_displayable(displayable))
    }

    async fn ensure_back_buffer(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: &mut ClassInstanceRef<Self>,
    ) -> Result<(ClassInstanceRef<Image>, ClassInstanceRef<Graphics>)> {
        let screen_width = context.screen_width().max(1);
        let screen_height = context.screen_height().max(1);

        let buffer: ClassInstanceRef<Image> = jvm.get_field(this, BACK_BUFFER_FIELD, BACK_BUFFER_DESC).await?;
        let graphics: ClassInstanceRef<Graphics> = jvm.get_field(this, BACK_GRAPHICS_FIELD, BACK_GRAPHICS_DESC).await?;
        if !buffer.is_null() && !graphics.is_null() {
            let (buffer_width, buffer_height, _) = Image::pixels(jvm, &buffer).await?;
            if buffer_width == screen_width && buffer_height == screen_height {
                return Ok((buffer, graphics));
            }
        }

        let buffer: ClassInstanceRef<Image> = jvm
            .new_class("javax/microedition/lcdui/Image", "(II)V", (screen_width, screen_height))
            .await?
            .into();
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.backBuffer.create class={} size={}x{}",
            this.class_definition().name(),
            screen_width,
            screen_height
        );

        let (_, _, mut pixels) = Image::pixels(jvm, &buffer).await?;
        jvm.store_array(&mut pixels, 0, vec![0xff00_0000u32 as i32; (screen_width * screen_height) as usize])
            .await?;

        let graphics = Self::new_back_graphics(jvm, &buffer).await?;
        jvm.put_field(this, BACK_BUFFER_FIELD, BACK_BUFFER_DESC, buffer.clone()).await?;
        jvm.put_field(this, BACK_GRAPHICS_FIELD, BACK_GRAPHICS_DESC, graphics.clone()).await?;

        Ok((buffer, graphics))
    }

    async fn new_back_graphics(jvm: &Jvm, buffer: &ClassInstanceRef<Image>) -> Result<ClassInstanceRef<Graphics>> {
        Ok(jvm
            .new_class(
                "javax/microedition/lcdui/Graphics",
                "(Ljavax/microedition/lcdui/Image;)V",
                (buffer.clone(),),
            )
            .await?
            .into())
    }

    async fn flush_back_buffer_region(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        buffer: &ClassInstanceRef<Image>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        if width <= 0 || height <= 0 {
            return Ok(());
        }

        let (buffer_width, buffer_height, pixels) = Image::pixels(jvm, buffer).await?;
        let src_x0 = x.max(0).min(buffer_width);
        let src_y0 = y.max(0).min(buffer_height);
        let src_x1 = (x + width).max(0).min(buffer_width);
        let src_y1 = (y + height).max(0).min(buffer_height);
        let copy_width = src_x1 - src_x0;
        let copy_height = src_y1 - src_y0;
        if copy_width <= 0 || copy_height <= 0 {
            return Ok(());
        }

        if publish_gpu_image_to_screen(
            &pixels,
            context.screen_width(),
            context.screen_height(),
            src_x0,
            src_y0,
            src_x0,
            src_y0,
            copy_width,
            copy_height,
            (0, 0, context.screen_width(), context.screen_height()),
        ) {
            tracing::info!(
                target: "rustjava_render",
                "gamecanvas.flush.gpu region={}x{}+{}+{} buffer={}x{}",
                copy_width,
                copy_height,
                src_x0,
                src_y0,
                buffer_width,
                buffer_height
            );
            return Ok(());
        }

        let pixels = jvm.load_array(&pixels, 0, (buffer_width * buffer_height) as usize).await?;
        tracing::info!(
            target: "rustjava_render",
            "gamecanvas.flush.copy region={}x{}+{}+{} buffer={}x{}",
            copy_width,
            copy_height,
            src_x0,
            src_y0,
            buffer_width,
            buffer_height
        );
        context.screen_draw_pixels_strided(src_x0, src_y0, copy_width, copy_height, &pixels, buffer_width, src_x0, src_y0, false);
        context.screen_present();

        Ok(())
    }
}

// class javax.microedition.lcdui.game.Sprite
impl Sprite {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/Sprite",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljavax/microedition/lcdui/Image;)V", Self::init_from_image, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljavax/microedition/lcdui/Image;II)V",
                    Self::init_from_frames,
                    Default::default(),
                ),
                JavaMethodProto::new("defineReferencePixel", "(II)V", Self::define_reference_pixel, Default::default()),
                JavaMethodProto::new("getFrame", "()I", Self::get_frame, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("getRawFrameCount", "()I", Self::get_raw_frame_count, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("nextFrame", "()V", Self::next_frame, Default::default()),
                JavaMethodProto::new("paint", "(Ljavax/microedition/lcdui/Graphics;)V", Self::paint, Default::default()),
                JavaMethodProto::new("setFrame", "(I)V", Self::set_frame, Default::default()),
                JavaMethodProto::new("setPosition", "(II)V", Self::set_position, Default::default()),
                JavaMethodProto::new("setRefPixelPosition", "(II)V", Self::set_ref_pixel_position, Default::default()),
                JavaMethodProto::new("setTransform", "(I)V", Self::set_transform, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("image", "Ljavax/microedition/lcdui/Image;", Default::default()),
                JavaFieldProto::new("frameWidth", "I", Default::default()),
                JavaFieldProto::new("frameHeight", "I", Default::default()),
                JavaFieldProto::new("frame", "I", Default::default()),
                JavaFieldProto::new("x", "I", Default::default()),
                JavaFieldProto::new("y", "I", Default::default()),
                JavaFieldProto::new("refX", "I", Default::default()),
                JavaFieldProto::new("refY", "I", Default::default()),
                JavaFieldProto::new("transform", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init_from_image(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, image: ClassInstanceRef<Image>) -> Result<()> {
        let (width, height, _) = if image.is_null() {
            (0, 0, ClassInstanceRef::new(None))
        } else {
            Image::pixels(jvm, &image).await?
        };
        Self::init_from_frames(jvm, context, this, image, width, height).await
    }

    async fn init_from_frames(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        image: ClassInstanceRef<Image>,
        frame_width: i32,
        frame_height: i32,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.game.Sprite::<init>({this:?}, {image:?}, {frame_width:?}, {frame_height:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        jvm.put_field(&mut this, "frameWidth", "I", frame_width.max(0)).await?;
        jvm.put_field(&mut this, "frameHeight", "I", frame_height.max(0)).await?;
        jvm.put_field(&mut this, "frame", "I", 0).await?;
        jvm.put_field(&mut this, "x", "I", 0).await?;
        jvm.put_field(&mut this, "y", "I", 0).await?;
        jvm.put_field(&mut this, "refX", "I", 0).await?;
        jvm.put_field(&mut this, "refY", "I", 0).await?;
        jvm.put_field(&mut this, "transform", "I", 0).await
    }

    async fn define_reference_pixel(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "refX", "I", x).await?;
        jvm.put_field(&mut this, "refY", "I", y).await
    }

    async fn get_frame(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "frame", "I").await
    }

    async fn get_height(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;
        let width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        Ok(if (4..=7).contains(&transform) { width } else { height })
    }

    async fn get_raw_frame_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Self::frame_count(jvm, &this).await
    }

    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;
        let width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        Ok(if (4..=7).contains(&transform) { height } else { width })
    }

    async fn next_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let frame: i32 = jvm.get_field(&this, "frame", "I").await?;
        let count = Self::frame_count(jvm, &this).await?.max(1);
        jvm.put_field(&mut this, "frame", "I", (frame + 1).rem_euclid(count)).await
    }

    async fn paint(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, graphics: ClassInstanceRef<Graphics>) -> Result<()> {
        if graphics.is_null() {
            return Ok(());
        }

        let image: ClassInstanceRef<Image> = jvm.get_field(&this, "image", "Ljavax/microedition/lcdui/Image;").await?;
        if image.is_null() {
            return Ok(());
        }

        let (image_width, _, _) = Image::pixels(jvm, &image).await?;
        let frame_width: i32 = jvm.get_field(&this, "frameWidth", "I").await?;
        let frame_height: i32 = jvm.get_field(&this, "frameHeight", "I").await?;
        if frame_width <= 0 || frame_height <= 0 {
            return Ok(());
        }

        let columns = (image_width / frame_width).max(1);
        let frame: i32 = jvm.get_field(&this, "frame", "I").await?;
        let src_x = frame.rem_euclid(columns) * frame_width;
        let src_y = (frame / columns) * frame_height;
        let x: i32 = jvm.get_field(&this, "x", "I").await?;
        let y: i32 = jvm.get_field(&this, "y", "I").await?;
        let transform: i32 = jvm.get_field(&this, "transform", "I").await?;

        Graphics::draw_region(
            jvm,
            context,
            graphics,
            image,
            src_x,
            src_y,
            frame_width,
            frame_height,
            transform,
            x,
            y,
            4 | 16,
        )
        .await
    }

    async fn set_frame(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, frame: i32) -> Result<()> {
        let count = Self::frame_count(jvm, &this).await?.max(1);
        jvm.put_field(&mut this, "frame", "I", frame.clamp(0, count - 1)).await
    }

    async fn set_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        jvm.put_field(&mut this, "x", "I", x).await?;
        jvm.put_field(&mut this, "y", "I", y).await
    }

    async fn set_ref_pixel_position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        let ref_x: i32 = jvm.get_field(&this, "refX", "I").await?;
        let ref_y: i32 = jvm.get_field(&this, "refY", "I").await?;
        jvm.put_field(&mut this, "x", "I", x - ref_x).await?;
        jvm.put_field(&mut this, "y", "I", y - ref_y).await
    }

    async fn set_transform(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, transform: i32) -> Result<()> {
        jvm.put_field(&mut this, "transform", "I", transform).await
    }

    async fn frame_count(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<i32> {
        let image: ClassInstanceRef<Image> = jvm.get_field(this, "image", "Ljavax/microedition/lcdui/Image;").await?;
        if image.is_null() {
            return Ok(0);
        }

        let (image_width, image_height, _) = Image::pixels(jvm, &image).await?;
        let frame_width: i32 = jvm.get_field(this, "frameWidth", "I").await?;
        let frame_height: i32 = jvm.get_field(this, "frameHeight", "I").await?;
        if frame_width <= 0 || frame_height <= 0 {
            return Ok(0);
        }

        Ok(((image_width / frame_width).max(1)) * ((image_height / frame_height).max(1)))
    }
}
