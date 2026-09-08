#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

impl CustomItem {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/CustomItem",
            parent_class: Some("javax/microedition/lcdui/Item"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("getMinContentWidth", "()I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getMinContentHeight", "()I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getPrefContentWidth", "(I)I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getPrefContentHeight", "(I)I", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("paint", "(Ljavax/microedition/lcdui/Graphics;II)V", MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("getGameAction", "(I)I", Self::get_game_action, Default::default()),
                JavaMethodProto::new("invalidate", "()V", Self::invalidate, Default::default()),
                JavaMethodProto::new("repaint", "()V", Self::repaint, Default::default()),
                JavaMethodProto::new("repaint", "(IIII)V", Self::repaint_region, Default::default()),
                JavaMethodProto::new("keyPressed", "(I)V", Self::key_pressed, Default::default()),
                JavaMethodProto::new("keyReleased", "(I)V", Self::key_released, Default::default()),
                JavaMethodProto::new("keyRepeated", "(I)V", Self::key_repeated, Default::default()),
                JavaMethodProto::new("pointerPressed", "(II)V", Self::pointer_pressed, Default::default()),
                JavaMethodProto::new("pointerReleased", "(II)V", Self::pointer_released, Default::default()),
                JavaMethodProto::new("pointerDragged", "(II)V", Self::pointer_dragged, Default::default()),
                JavaMethodProto::new("showNotify", "()V", Self::show_notify, Default::default()),
                JavaMethodProto::new("hideNotify", "()V", Self::hide_notify, Default::default()),
                JavaMethodProto::new("sizeChanged", "(II)V", Self::size_changed, Default::default()),
                JavaMethodProto::new("traverse", "(IIII[I)Z", Self::traverse, Default::default()),
                JavaMethodProto::new("traverseOut", "()V", Self::traverse_out, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("KEY_PRESS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_RELEASE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("KEY_REPEAT", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POINTER_PRESS", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POINTER_RELEASE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("POINTER_DRAG", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("NONE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRAVERSE_HORIZONTAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TRAVERSE_VERTICAL", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/lcdui/CustomItem";
        jvm.put_static_field(class, "KEY_PRESS", "I", 4).await?;
        jvm.put_static_field(class, "KEY_RELEASE", "I", 8).await?;
        jvm.put_static_field(class, "KEY_REPEAT", "I", 16).await?;
        jvm.put_static_field(class, "POINTER_PRESS", "I", 32).await?;
        jvm.put_static_field(class, "POINTER_RELEASE", "I", 64).await?;
        jvm.put_static_field(class, "POINTER_DRAG", "I", 128).await?;
        jvm.put_static_field(class, "NONE", "I", 0).await?;
        jvm.put_static_field(class, "TRAVERSE_HORIZONTAL", "I", 1).await?;
        jvm.put_static_field(class, "TRAVERSE_VERTICAL", "I", 2).await
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, label: ClassInstanceRef<String>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/lcdui/Item", "<init>", "(Ljava/lang/String;)V", (label,))
            .await
    }

    pub(super) async fn get_game_action(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, key_code: i32) -> Result<i32> {
        Ok(match key_code {
            -1 => 1,
            -2 => 6,
            -3 => 2,
            -4 => 5,
            -5 => 8,
            _ => 0,
        })
    }

    pub(super) async fn invalidate(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.CustomItem::invalidate({this:?})");
        Ok(())
    }

    pub(super) async fn repaint(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.CustomItem::repaint({this:?})");
        Ok(())
    }

    pub(super) async fn repaint_region(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _x: i32,
        _y: i32,
        _w: i32,
        _h: i32,
    ) -> Result<()> {
        tracing::trace!("javax.microedition.lcdui.CustomItem::repaint({this:?})");
        Ok(())
    }

    stub_void! {
        key_pressed(_key: i32);
        key_released(_key: i32);
        key_repeated(_key: i32);
        pointer_pressed(_x: i32, _y: i32);
        pointer_released(_x: i32, _y: i32);
        pointer_dragged(_x: i32, _y: i32);
        show_notify();
        hide_notify();
        size_changed(_w: i32, _h: i32);
        traverse_out()
    }

    pub(super) async fn traverse(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _dir: i32,
        _vw: i32,
        _vh: i32,
        _vis: ClassInstanceRef<Array<i32>>,
    ) -> Result<bool> {
        Ok(false)
    }
}
