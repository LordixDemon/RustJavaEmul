use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct Player;

impl Player {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sprintpcs/media/Player",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("play", "()V", Self::play, Default::default()),
                JavaMethodProto::new("play", "(Lcom/sprintpcs/media/Clip;I)V", Self::play_clip, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new("pause", "()V", Self::pause, Default::default()),
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
        play_clip(_clip: ClassInstanceRef<Clip>, _loop: i32);
        stop();
        pause();
    }
    async fn is_supported(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }
}

pub struct Muglet;

impl Muglet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sprintpcs/util/Muglet",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getMuglet", "()Lcom/sprintpcs/util/Muglet;", Self::get_muglet, MethodAccessFlags::STATIC),
                JavaMethodProto::new("getURI", "()Ljava/lang/String;", Self::get_uri, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn get_muglet(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("com/sprintpcs/util/Muglet", "()V", ()).await?.into())
    }

    async fn get_uri(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        JavaLangString::from_rust_string(jvm, "").await.map(Into::into)
    }
}

pub struct Clip;
pub struct Vibrator;
pub struct SprintSystem;

impl Clip {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sprintpcs/media/Clip",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "([BLjava/lang/String;II)V",
                Self::init,
                Default::default(),
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _data: ClassInstanceRef<jvm::Array<i8>>,
        _type: ClassInstanceRef<String>,
        _priority: i32,
        _volume: i32,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

impl Vibrator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sprintpcs/media/Vibrator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("vibrate", "(I)V", Self::vibrate, MethodAccessFlags::STATIC)],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn vibrate(_: &Jvm, context: &mut RuntimeContext, duration: i32) -> Result<()> {
        context.start_vibra(100, duration as i64);
        Ok(())
    }
}

impl SprintSystem {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/sprintpcs/util/System",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "setExitURI",
                "(Ljava/lang/String;)V",
                Self::set_exit_uri,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn set_exit_uri(_: &Jvm, _: &mut RuntimeContext, _uri: ClassInstanceRef<String>) -> Result<()> {
        Ok(())
    }
}

pub mod media {
    pub use super::{Clip, Player, Vibrator};
}

pub mod util {
    pub use super::{Muglet, SprintSystem as System};
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Player, Muglet, Clip, Vibrator, SprintSystem]
}
