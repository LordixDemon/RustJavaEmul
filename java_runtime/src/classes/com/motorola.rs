use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct FunLight;
pub struct FunLightRegion;

impl FunLight {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/motorola/funlight/FunLight",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("getControl", "()I", Self::get_control, MethodAccessFlags::STATIC),
                JavaMethodProto::new("releaseControl", "()V", Self::release_control, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getRegion",
                    "(I)Lcom/motorola/funlight/FunLightRegion;",
                    Self::get_region,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getRegions",
                    "()[Lcom/motorola/funlight/FunLightRegion;",
                    Self::get_regions,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("BLANK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("OFF", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ON", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BLACK", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("BLUE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CYAN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("GREEN", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("MAGENTA", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("RED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("WHITE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("YELLOW", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "com/motorola/funlight/FunLight";
        jvm.put_static_field(class, "BLANK", "I", 0).await?;
        jvm.put_static_field(class, "OFF", "I", 0).await?;
        jvm.put_static_field(class, "ON", "I", 1).await?;
        jvm.put_static_field(class, "BLACK", "I", 0).await?;
        jvm.put_static_field(class, "BLUE", "I", 0x0000ff).await?;
        jvm.put_static_field(class, "CYAN", "I", 0x00ffff).await?;
        jvm.put_static_field(class, "GREEN", "I", 0x00ff00).await?;
        jvm.put_static_field(class, "MAGENTA", "I", 0xff00ff).await?;
        jvm.put_static_field(class, "RED", "I", 0xff0000).await?;
        jvm.put_static_field(class, "WHITE", "I", 0xffffff).await?;
        jvm.put_static_field(class, "YELLOW", "I", 0xffff00).await
    }

    async fn get_control(_: &Jvm, _: &mut RuntimeContext) -> Result<i32> {
        Ok(1)
    }
    async fn release_control(_: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        Ok(())
    }
    async fn get_region(jvm: &Jvm, _: &mut RuntimeContext, id: i32) -> Result<ClassInstanceRef<FunLightRegion>> {
        Ok(jvm.new_class("com/motorola/funlight/FunLightRegion", "(I)V", (id,)).await?.into())
    }
    async fn get_regions(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<jvm::Array<ClassInstanceRef<FunLightRegion>>>> {
        let mut array = jvm.instantiate_array("Lcom/motorola/funlight/FunLightRegion;", 1).await?;
        let region = jvm.new_class("com/motorola/funlight/FunLightRegion", "(I)V", (0,)).await?;
        jvm.store_array(&mut array, 0, vec![region]).await?;
        Ok(array.into())
    }
}

impl FunLightRegion {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/motorola/funlight/FunLightRegion",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, Default::default()),
                JavaMethodProto::new("getID", "()I", Self::get_id, Default::default()),
                JavaMethodProto::new("getColor", "()I", Self::get_color, Default::default()),
                JavaMethodProto::new("setColor", "(I)I", Self::set_color, Default::default()),
                JavaMethodProto::new("getControl", "()I", Self::get_control, Default::default()),
                JavaMethodProto::new("releaseControl", "()V", Self::release_control, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("id", "I", Default::default()),
                JavaFieldProto::new("color", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, id: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "id", "I", id).await?;
        jvm.put_field(&mut this, "color", "I", 0).await
    }
    async fn get_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "id", "I").await
    }
    async fn get_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "color", "I").await
    }
    async fn set_color(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<i32> {
        context.set_lights(0, if color == 0 { 0 } else { 100 });
        jvm.put_field(&mut this, "color", "I", color).await?;
        Ok(color)
    }
    async fn get_control(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }
    async fn release_control(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}

pub mod funlight {
    pub use super::{FunLight, FunLightRegion};
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![FunLight, FunLightRegion]
}
