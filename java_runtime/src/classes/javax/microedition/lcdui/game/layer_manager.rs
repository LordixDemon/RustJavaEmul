#[allow(unused_imports)]
use super::*;
use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Graphics};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

impl LayerManager {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/lcdui/game/LayerManager",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("append", "(Ljavax/microedition/lcdui/game/Layer;)V", Self::append, Default::default()),
                JavaMethodProto::new("insert", "(Ljavax/microedition/lcdui/game/Layer;I)V", Self::insert, Default::default()),
                JavaMethodProto::new(
                    "getLayerAt",
                    "(I)Ljavax/microedition/lcdui/game/Layer;",
                    Self::get_layer_at,
                    Default::default(),
                ),
                JavaMethodProto::new("getSize", "()I", Self::get_size, Default::default()),
                JavaMethodProto::new("remove", "(Ljavax/microedition/lcdui/game/Layer;)V", Self::remove, Default::default()),
                JavaMethodProto::new("paint", "(Ljavax/microedition/lcdui/Graphics;II)V", Self::paint, Default::default()),
                JavaMethodProto::new("setViewWindow", "(IIII)V", Self::set_view_window, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("layers", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("viewX", "I", Default::default()),
                JavaFieldProto::new("viewY", "I", Default::default()),
                JavaFieldProto::new("viewW", "I", Default::default()),
                JavaFieldProto::new("viewH", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let layers = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "layers", "Ljava/util/Vector;", layers).await?;
        jvm.put_field(&mut this, "viewX", "I", 0).await?;
        jvm.put_field(&mut this, "viewY", "I", 0).await?;
        jvm.put_field(&mut this, "viewW", "I", 0).await?;
        jvm.put_field(&mut this, "viewH", "I", 0).await
    }

    pub(super) async fn append(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, layer: ClassInstanceRef<Layer>) -> Result<()> {
        let layers: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&this, "layers", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&layers, "addElement", "(Ljava/lang/Object;)V", (layer,)).await
    }

    pub(super) async fn insert(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        layer: ClassInstanceRef<Layer>,
        index: i32,
    ) -> Result<()> {
        let layers: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&this, "layers", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&layers, "insertElementAt", "(Ljava/lang/Object;I)V", (layer, index))
            .await
    }

    pub(super) async fn get_layer_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Layer>> {
        let layers: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&this, "layers", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&layers, "elementAt", "(I)Ljava/lang/Object;", (index,)).await
    }

    pub(super) async fn get_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let layers: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&this, "layers", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&layers, "size", "()I", ()).await
    }

    pub(super) async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, layer: ClassInstanceRef<Layer>) -> Result<()> {
        let layers: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&this, "layers", "Ljava/util/Vector;").await?;
        let _: bool = jvm.invoke_virtual(&layers, "removeElement", "(Ljava/lang/Object;)Z", (layer,)).await?;
        Ok(())
    }

    pub(super) async fn paint(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        graphics: ClassInstanceRef<Graphics>,
        x: i32,
        y: i32,
    ) -> Result<()> {
        if graphics.is_null() {
            return Ok(());
        }
        let size = Self::get_size(jvm, context, this.clone()).await?;
        let view_x: i32 = jvm.get_field(&this, "viewX", "I").await?;
        let view_y: i32 = jvm.get_field(&this, "viewY", "I").await?;
        for i in (0..size).rev() {
            let layer = Self::get_layer_at(jvm, context, this.clone(), i).await?;
            if layer.is_null() {
                continue;
            }
            let visible: bool = jvm.invoke_virtual(&layer, "isVisible", "()Z", ()).await?;
            if !visible {
                continue;
            }
            let lx: i32 = jvm.invoke_virtual(&layer, "getX", "()I", ()).await?;
            let ly: i32 = jvm.invoke_virtual(&layer, "getY", "()I", ()).await?;
            let _: () = jvm
                .invoke_virtual(&layer, "setPosition", "(II)V", (lx - view_x + x, ly - view_y + y))
                .await?;
            let _: () = jvm
                .invoke_virtual(&layer, "paint", "(Ljavax/microedition/lcdui/Graphics;)V", (graphics.clone(),))
                .await?;
            let _: () = jvm.invoke_virtual(&layer, "setPosition", "(II)V", (lx, ly)).await?;
        }
        Ok(())
    }

    pub(super) async fn set_view_window(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "viewX", "I", x).await?;
        jvm.put_field(&mut this, "viewY", "I", y).await?;
        jvm.put_field(&mut this, "viewW", "I", width).await?;
        jvm.put_field(&mut this, "viewH", "I", height).await
    }
}
