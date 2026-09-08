use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct Sound;

impl Sound {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/sound/Sound",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(II)V", Self::init_tone, Default::default()),
                JavaMethodProto::new("<init>", "(IJ)V", Self::init_tone_long, Default::default()),
                JavaMethodProto::new("<init>", "([BI)V", Self::init_data, Default::default()),
                JavaMethodProto::new("init", "(II)V", Self::reinit_tone, Default::default()),
                JavaMethodProto::new("init", "([BI)V", Self::reinit_data, Default::default()),
                JavaMethodProto::new("play", "(I)V", Self::play, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new("resume", "()V", Self::resume, Default::default()),
                JavaMethodProto::new("release", "()V", Self::release, Default::default()),
                JavaMethodProto::new("getState", "()I", Self::get_state, Default::default()),
                JavaMethodProto::new("getGain", "()I", Self::get_gain, Default::default()),
                JavaMethodProto::new("setGain", "(I)I", Self::set_gain, Default::default()),
                JavaMethodProto::new("setGain", "(I)V", Self::set_gain_void, Default::default()),
                JavaMethodProto::new(
                    "setSoundListener",
                    "(Lcom/nokia/mid/sound/SoundListener;)V",
                    Self::set_sound_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("getConcurrentSoundCount", "(I)I", Self::concurrent, MethodAccessFlags::STATIC),
                JavaMethodProto::new("getSupportedFormats", "()[I", Self::formats, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("state", "I", Default::default()),
                JavaFieldProto::new("gain", "I", Default::default()),
                JavaFieldProto::new("listener", "Lcom/nokia/mid/sound/SoundListener;", Default::default()),
                JavaFieldProto::new("SOUND_PLAYING", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SOUND_STOPPED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("SOUND_UNINITIALIZED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FORMAT_TONE", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("FORMAT_WAV", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "com/nokia/mid/sound/Sound";
        jvm.put_static_field(class, "SOUND_PLAYING", "I", 0).await?;
        jvm.put_static_field(class, "SOUND_STOPPED", "I", 1).await?;
        jvm.put_static_field(class, "SOUND_UNINITIALIZED", "I", 3).await?;
        jvm.put_static_field(class, "FORMAT_TONE", "I", 1).await?;
        jvm.put_static_field(class, "FORMAT_WAV", "I", 5).await
    }

    async fn init_tone(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, freq: i32, duration: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "gain", "I", 255).await?;
        jvm.put_field(&mut this, "state", "I", 1).await?;
        context.play_tone(freq, duration, 100);
        Ok(())
    }

    async fn init_tone_long(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, freq: i32, duration: i64) -> Result<()> {
        Self::init_tone(jvm, context, this, freq, duration as i32).await
    }

    async fn init_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _data: ClassInstanceRef<Array<i8>>,
        _format: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "gain", "I", 255).await?;
        jvm.put_field(&mut this, "state", "I", 1).await
    }

    async fn reinit_tone(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, freq: i32, duration: i32) -> Result<()> {
        context.play_tone(freq, duration, 100);
        jvm.put_field(&mut this, "state", "I", 1).await
    }

    async fn reinit_data(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        _data: ClassInstanceRef<Array<i8>>,
        _format: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 1).await
    }

    async fn play(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _loop: i32) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 0).await
    }
    async fn stop(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 1).await
    }
    async fn resume(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::play(jvm, context, this, 1).await
    }
    async fn release(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 3).await
    }
    async fn get_state(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "state", "I").await
    }
    async fn get_gain(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "gain", "I").await
    }
    async fn set_gain(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, gain: i32) -> Result<i32> {
        let gain = gain.clamp(0, 255);
        jvm.put_field(&mut this, "gain", "I", gain).await?;
        Ok(gain)
    }
    async fn set_gain_void(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, gain: i32) -> Result<()> {
        let _: i32 = Self::set_gain(jvm, context, this, gain).await?;
        Ok(())
    }
    async fn set_sound_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<SoundListener>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "listener", "Lcom/nokia/mid/sound/SoundListener;", listener)
            .await
    }
    async fn concurrent(_: &Jvm, _: &mut RuntimeContext, _format: i32) -> Result<i32> {
        Ok(1)
    }
    async fn formats(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<i32>>> {
        let mut array = jvm.instantiate_array("I", 2).await?;
        jvm.store_array(&mut array, 0, alloc::vec![1, 5]).await?;
        Ok(array.into())
    }
}

pub struct SoundListener;

impl SoundListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/nokia/mid/sound/SoundListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Sound, SoundListener]
}
