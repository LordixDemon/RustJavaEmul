use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct Vibration;
pub struct AudioClip;

impl Vibration {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/samsung/util/Vibration",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("start", "(II)V", Self::start, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stop", "()V", Self::stop, MethodAccessFlags::STATIC),
                JavaMethodProto::new("isSupported", "()Z", Self::is_supported, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn start(_: &Jvm, context: &mut RuntimeContext, duration: i32, _strength: i32) -> Result<()> {
        context.start_vibra(100, duration as i64);
        Ok(())
    }
    async fn stop(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        context.stop_vibra();
        Ok(())
    }
    async fn is_supported(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }
}

impl AudioClip {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/samsung/util/AudioClip",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(ILjava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I[BII)V", Self::init_bytes, Default::default()),
                JavaMethodProto::new("play", "(II)V", Self::play, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new("pause", "()V", Self::pause, Default::default()),
                JavaMethodProto::new("resume", "()V", Self::resume, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _t: i32,
        _n: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn init_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _t: i32,
        _data: ClassInstanceRef<jvm::Array<i8>>,
        _offset: i32,
        _length: i32,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    stub_void! {
        play(_loop: i32, _vol: i32);
        stop();
        pause();
        resume();
    }
}

pub struct LCDLight;
pub struct SM;
pub struct SMS;

impl LCDLight {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/samsung/util/LCDLight",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("isSupported", "()Z", Self::is_supported, MethodAccessFlags::STATIC),
                JavaMethodProto::new("on", "(I)V", Self::on, MethodAccessFlags::STATIC),
                JavaMethodProto::new("off", "()V", Self::off, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn is_supported(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }

    async fn on(_: &Jvm, _: &mut RuntimeContext, _color: i32) -> Result<()> {
        Ok(())
    }

    async fn off(_: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        Ok(())
    }
}

impl SM {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/samsung/util/SM",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_with_data,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setDestAddress",
                    "(Ljava/lang/String;)Lcom/samsung/util/SM;",
                    Self::set_dest_address_ret,
                    Default::default(),
                ),
                JavaMethodProto::new("setDestAddress", "(Ljava/lang/String;)V", Self::set_dest_address, Default::default()),
                JavaMethodProto::new("setData", "(Ljava/lang/String;)V", Self::set_data, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn init_with_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _dest: ClassInstanceRef<crate::classes::java::lang::String>,
        _callback: ClassInstanceRef<crate::classes::java::lang::String>,
        _data: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn set_dest_address_ret(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _dest: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<ClassInstanceRef<Self>> {
        Ok(this)
    }

    async fn set_dest_address(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _dest: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        Ok(())
    }

    async fn set_data(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _data: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        Ok(())
    }
}

impl SMS {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/samsung/util/SMS",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("isSupported", "()Z", Self::is_supported, MethodAccessFlags::STATIC),
                JavaMethodProto::new("send", "(Lcom/samsung/util/SM;)V", Self::send, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn is_supported(_: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        Ok(true)
    }

    async fn send(_: &Jvm, _: &mut RuntimeContext, _sm: ClassInstanceRef<SM>) -> Result<()> {
        Ok(())
    }
}

pub mod util {
    pub use super::{AudioClip, LCDLight, SM, SMS, Vibration};
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Vibration, AudioClip, LCDLight, SM, SMS]
}
