use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{java::lang::String, javax::microedition::lcdui::Graphics},
};

// class javax.microedition.lcdui.Canvas
pub struct Canvas;

impl Canvas {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/Canvas",
            parent_class: Some("javax/microedition/lcdui/Displayable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("repaint", "()V", Self::repaint, Default::default()),
                JavaMethodProto::new("repaint", "(IIII)V", Self::repaint_region, Default::default()),
                JavaMethodProto::new("serviceRepaints", "()V", Self::service_repaints, Default::default()),
                JavaMethodProto::new("setFullScreenMode", "(Z)V", Self::set_full_screen_mode, Default::default()),
                JavaMethodProto::new("isDoubleBuffered", "()Z", Self::is_double_buffered, Default::default()),
                JavaMethodProto::new("hasPointerEvents", "()Z", Self::has_pointer_events, Default::default()),
                JavaMethodProto::new("hasPointerMotionEvents", "()Z", Self::has_pointer_motion_events, Default::default()),
                JavaMethodProto::new("hasRepeatEvents", "()Z", Self::has_repeat_events, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new("getGameAction", "(I)I", Self::get_game_action, Default::default()),
                JavaMethodProto::new("getKeyCode", "(I)I", Self::get_key_code, Default::default()),
                JavaMethodProto::new("getKeyName", "(I)Ljava/lang/String;", Self::get_key_name, Default::default()),
                JavaMethodProto::new("keyPressed", "(I)V", Self::key_pressed, Default::default()),
                JavaMethodProto::new("keyReleased", "(I)V", Self::key_released, Default::default()),
                JavaMethodProto::new("keyRepeated", "(I)V", Self::key_repeated, Default::default()),
                JavaMethodProto::new("pointerPressed", "(II)V", Self::pointer_pressed, Default::default()),
                JavaMethodProto::new("pointerReleased", "(II)V", Self::pointer_released, Default::default()),
                JavaMethodProto::new("pointerDragged", "(II)V", Self::pointer_dragged, Default::default()),
                JavaMethodProto::new("sizeChanged", "(II)V", Self::size_changed, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("UP", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LEFT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RIGHT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("DOWN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FIRE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_A", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_B", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_C", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GAME_D", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM0", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM1", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM2", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM3", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM4", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM5", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM6", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM7", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM8", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_NUM9", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_STAR", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_POUND", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/Canvas";
        jvm.put_static_field(class, "UP", "I", 1).await?;
        jvm.put_static_field(class, "LEFT", "I", 2).await?;
        jvm.put_static_field(class, "RIGHT", "I", 5).await?;
        jvm.put_static_field(class, "DOWN", "I", 6).await?;
        jvm.put_static_field(class, "FIRE", "I", 8).await?;
        jvm.put_static_field(class, "GAME_A", "I", 9).await?;
        jvm.put_static_field(class, "GAME_B", "I", 10).await?;
        jvm.put_static_field(class, "GAME_C", "I", 11).await?;
        jvm.put_static_field(class, "GAME_D", "I", 12).await?;
        jvm.put_static_field(class, "KEY_NUM0", "I", b'0' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM1", "I", b'1' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM2", "I", b'2' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM3", "I", b'3' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM4", "I", b'4' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM5", "I", b'5' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM6", "I", b'6' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM7", "I", b'7' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM8", "I", b'8' as i32).await?;
        jvm.put_static_field(class, "KEY_NUM9", "I", b'9' as i32).await?;
        jvm.put_static_field(class, "KEY_STAR", "I", b'*' as i32).await?;
        jvm.put_static_field(class, "KEY_POUND", "I", b'#' as i32).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Canvas::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "javax/microedition/lcdui/Displayable", "<init>", "()V", ())
            .await?;

        Ok(())
    }

    async fn repaint(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::repaint({this:?})");

        let class_name = this.class_definition().name();
        let Some(displayable) = this.instance.as_deref() else {
            return Ok(());
        };
        let is_current = context.is_current_displayable(displayable);
        tracing::info!(target: "rustjava_render", "canvas.repaint class={} current={}", class_name, is_current);
        if !is_current {
            return Ok(());
        }

        let paints_to_back_buffer = jvm.is_instance(displayable, "javax/microedition/lcdui/game/GameCanvas");
        let graphics: ClassInstanceRef<Graphics> = if paints_to_back_buffer {
            jvm.invoke_virtual(&this, "getGraphics", "()Ljavax/microedition/lcdui/Graphics;", ())
                .await?
        } else {
            jvm.new_class("javax/microedition/lcdui/Graphics", "()V", ()).await?.into()
        };
        tracing::info!(
            target: "rustjava_render",
            "canvas.repaint.paint class={} target={}",
            class_name,
            if paints_to_back_buffer { "gamecanvas-backbuffer" } else { "screen" }
        );

        let _: () = jvm
            .invoke_virtual(&this, "paint", "(Ljavax/microedition/lcdui/Graphics;)V", (graphics,))
            .await?;

        if jvm
            .get_class(&class_name)
            .is_some_and(|class| class.definition.field("f", "Z", true).is_some())
        {
            jvm.put_static_field(&class_name, "f", "Z", true).await?;
        }
        if paints_to_back_buffer {
            tracing::info!(target: "rustjava_render", "canvas.repaint.flushGameCanvas class={}", class_name);
            let _: () = jvm.invoke_virtual(&this, "flushGraphics", "()V", ()).await?;
        } else {
            tracing::info!(target: "rustjava_render", "canvas.repaint.present class={}", class_name);
            context.screen_present();
        }

        Ok(())
    }

    async fn repaint_region(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::repaint({this:?}, {x:?}, {y:?}, {width:?}, {height:?})");

        let _: () = jvm.invoke_virtual(&this, "repaint", "()V", ()).await?;
        Ok(())
    }

    async fn service_repaints(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::serviceRepaints({this:?})");

        let _: () = jvm.invoke_virtual(&this, "repaint", "()V", ()).await?;

        Ok(())
    }

    async fn set_full_screen_mode(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mode: bool) -> Result<()> {
        tracing::debug!("javax.microedition.lcdui.Canvas::setFullScreenMode({this:?}, {mode:?})");

        Ok(())
    }

    async fn is_double_buffered(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Canvas::isDoubleBuffered({this:?})");

        Ok(true)
    }

    async fn has_pointer_events(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Canvas::hasPointerEvents({this:?})");

        Ok(false)
    }

    async fn has_pointer_motion_events(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Canvas::hasPointerMotionEvents({this:?})");

        Ok(false)
    }

    async fn has_repeat_events(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::trace!("javax.microedition.lcdui.Canvas::hasRepeatEvents({this:?})");

        Ok(false)
    }

    async fn get_width(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Canvas::getWidth({this:?})");

        Ok(context.screen_width())
    }

    async fn get_height(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Canvas::getHeight({this:?})");

        Ok(context.screen_height())
    }

    async fn get_game_action(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key_code: i32) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Canvas::getGameAction({this:?}, {key_code:?})");

        Ok(match key_code {
            -1 => 1,
            -3 => 2,
            -4 => 5,
            -2 => 6,
            -5 => 8,
            -6 | 49 => 9,
            -7 | 51 => 10,
            50 => 1,
            52 => 2,
            54 => 5,
            56 => 6,
            53 => 8,
            55 => 11,
            57 => 12,
            _ => 0,
        })
    }

    async fn get_key_code(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, game_action: i32) -> Result<i32> {
        tracing::trace!("javax.microedition.lcdui.Canvas::getKeyCode({this:?}, {game_action:?})");

        Ok(match game_action {
            1 => -1,
            2 => -3,
            5 => -4,
            6 => -2,
            8 => -5,
            9 => -6,
            10 => -7,
            11 => 55,
            12 => 57,
            _ => 0,
        })
    }

    async fn get_key_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key_code: i32) -> Result<ClassInstanceRef<String>> {
        tracing::trace!("javax.microedition.lcdui.Canvas::getKeyName({this:?}, {key_code:?})");

        let name = match key_code {
            -1 => "UP",
            -2 => "DOWN",
            -3 => "LEFT",
            -4 => "RIGHT",
            -5 => "FIRE",
            -6 => "SOFT1",
            -7 => "SOFT2",
            35 => "#",
            42 => "*",
            48 => "0",
            49 => "1",
            50 => "2",
            51 => "3",
            52 => "4",
            53 => "5",
            54 => "6",
            55 => "7",
            56 => "8",
            57 => "9",
            _ => "",
        };

        Ok(JavaLangString::from_rust_string(jvm, name).await?.into())
    }

    async fn key_pressed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key_code: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::keyPressed({this:?}, {key_code:?})");

        Ok(())
    }

    async fn key_released(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key_code: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::keyReleased({this:?}, {key_code:?})");

        Ok(())
    }

    async fn key_repeated(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key_code: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::keyRepeated({this:?}, {key_code:?})");

        Ok(())
    }

    async fn pointer_pressed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::pointerPressed({this:?}, {x:?}, {y:?})");

        Ok(())
    }

    async fn pointer_released(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::pointerReleased({this:?}, {x:?}, {y:?})");

        Ok(())
    }

    async fn pointer_dragged(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, x: i32, y: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::pointerDragged({this:?}, {x:?}, {y:?})");

        Ok(())
    }

    async fn size_changed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, width: i32, height: i32) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.Canvas::sizeChanged({this:?}, {width:?}, {height:?})");

        Ok(())
    }
}
