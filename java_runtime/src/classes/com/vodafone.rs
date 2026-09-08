use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::javax::microedition::lcdui::Image};

pub struct Sound;

impl Sound {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/vodafone/v10/Sound",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("play", "()V", Self::play, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new("isSupported", "()Z", Self::is_supported, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    stub_void! {
        play();
        stop();
    }
    async fn is_supported(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }
}

pub struct ImageEncoder;

impl ImageEncoder {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/vodafone/util/ImageEncoder",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "createEncoder",
                    "(I)Lcom/vodafone/util/ImageEncoder;",
                    Self::create_encoder,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "encodeOffscreen",
                    "(Ljavax/microedition/lcdui/Image;IIII)[B",
                    Self::encode_offscreen,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn create_encoder(jvm: &Jvm, _: &mut RuntimeContext, _format: i32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("com/vodafone/util/ImageEncoder", "()V", ()).await?.into())
    }

    async fn encode_offscreen(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _image: ClassInstanceRef<Image>,
        _x: i32,
        _y: i32,
        _width: i32,
        _height: i32,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        Ok(jvm.instantiate_array("B", 0).await?.into())
    }
}

pub mod v10 {
    pub use super::Sound;
}

pub mod util {
    pub use super::ImageEncoder;
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Sound, ImageEncoder]
}
