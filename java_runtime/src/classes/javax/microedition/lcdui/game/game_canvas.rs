#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::javax::microedition::lcdui::{Graphics, Image, pace_game_frame, publish_gpu_image_to_screen},
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

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
                JavaMethodProto::new("paint", "(Ljavax/microedition/lcdui/Graphics;)V", Self::paint, Default::default()),
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

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
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

    pub(super) async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, suppress_key_events: bool) -> Result<()> {
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

    pub(super) async fn get_graphics(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Graphics>> {
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

    pub(super) async fn flush_graphics(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
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

    pub(super) async fn flush_graphics_region(
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

    pub(super) async fn get_key_states(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.game.GameCanvas::getKeyStates({this:?})");

        Ok(context.game_key_states())
    }

    pub(super) async fn paint(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _graphics: ClassInstanceRef<Graphics>) -> Result<()> {
        Ok(())
    }

    pub(super) fn is_current_canvas(context: &RuntimeContext, this: &ClassInstanceRef<Self>) -> bool {
        this.instance
            .as_deref()
            .is_some_and(|displayable| context.is_current_displayable(displayable))
    }

    pub(super) async fn ensure_back_buffer(
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

    pub(super) async fn new_back_graphics(jvm: &Jvm, buffer: &ClassInstanceRef<Image>) -> Result<ClassInstanceRef<Graphics>> {
        Ok(jvm
            .new_class(
                "javax/microedition/lcdui/Graphics",
                "(Ljavax/microedition/lcdui/Image;)V",
                (buffer.clone(),),
            )
            .await?
            .into())
    }

    pub(super) async fn flush_back_buffer_region(
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

        pace_game_frame(context).await;

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

        let _ = publish_gpu_image_to_screen(
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
        );

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
