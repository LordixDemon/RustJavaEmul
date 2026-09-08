use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// interface javax.microedition.media.Control
pub struct Control;

impl Control {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/Control",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}

// class javax.microedition.media.control.VolumeControl
pub struct VolumeControl;

impl VolumeControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/VolumeControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getLevel", "()I", Self::get_level, Default::default()),
                JavaMethodProto::new("isMuted", "()Z", Self::is_muted, Default::default()),
                JavaMethodProto::new("setLevel", "(I)I", Self::set_level, Default::default()),
                JavaMethodProto::new("setMute", "(Z)V", Self::set_mute, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("level", "I", Default::default()),
                JavaFieldProto::new("muted", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "level", "I", 100).await?;
        jvm.put_field(&mut this, "muted", "Z", false).await?;

        Ok(())
    }

    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::getLevel({this:?})");

        jvm.get_field(&this, "level", "I").await
    }

    async fn is_muted(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::isMuted({this:?})");

        jvm.get_field(&this, "muted", "Z").await
    }

    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: i32) -> Result<i32> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::setLevel({this:?}, {level:?})");

        let level = level.clamp(0, 100);
        jvm.put_field(&mut this, "level", "I", level).await?;

        Ok(level)
    }

    async fn set_mute(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, muted: bool) -> Result<()> {
        tracing::debug!("javax.microedition.media.control.VolumeControl::setMute({this:?}, {muted:?})");

        jvm.put_field(&mut this, "muted", "Z", muted).await?;
        Ok(())
    }
}

pub struct ToneControl;
pub struct MIDIControl;
pub struct StopTimeControl;
pub struct MetaDataControl;
pub struct TempoControl;
pub struct PitchControl;

impl TempoControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/TempoControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getTempo", "()I", Self::get_tempo, Default::default()),
                JavaMethodProto::new("setTempo", "(I)I", Self::set_tempo, Default::default()),
                JavaMethodProto::new("getRate", "()I", Self::get_tempo, Default::default()),
                JavaMethodProto::new("setRate", "(I)I", Self::set_tempo, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("tempo", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "tempo", "I", 100_000).await?;
        Ok(())
    }

    async fn get_tempo(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "tempo", "I").await
    }

    async fn set_tempo(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tempo: i32) -> Result<i32> {
        let tempo = tempo.clamp(10_000, 300_000);
        jvm.put_field(&mut this, "tempo", "I", tempo).await?;
        Ok(tempo)
    }
}

impl PitchControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/PitchControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getPitch", "()I", Self::get_pitch, Default::default()),
                JavaMethodProto::new("setPitch", "(I)I", Self::set_pitch, Default::default()),
                JavaMethodProto::new("getMaxPitch", "()I", Self::get_max_pitch, Default::default()),
                JavaMethodProto::new("getMinPitch", "()I", Self::get_min_pitch, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("pitch", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "pitch", "I", 0).await?;
        Ok(())
    }

    async fn get_pitch(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "pitch", "I").await
    }

    async fn set_pitch(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pitch: i32) -> Result<i32> {
        let pitch = pitch.clamp(-12000, 12000);
        jvm.put_field(&mut this, "pitch", "I", pitch).await?;
        Ok(pitch)
    }

    async fn get_max_pitch(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(12000)
    }

    async fn get_min_pitch(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(-12000)
    }
}

impl ToneControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/ToneControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("setSequence", "([B)V", Self::set_sequence, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn set_sequence(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _sequence: ClassInstanceRef<jvm::Array<i8>>) -> Result<()> {
        tracing::debug!("javax.microedition.media.control.ToneControl::setSequence({this:?})");
        Ok(())
    }
}

impl MIDIControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/MIDIControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("isBankQuerySupported", "()Z", Self::bank_query, Default::default()),
                JavaMethodProto::new("getChannelVolume", "(I)I", Self::get_channel_volume, Default::default()),
                JavaMethodProto::new("setChannelVolume", "(II)V", Self::set_channel_volume, Default::default()),
                JavaMethodProto::new("getProgram", "(I)[I", Self::get_program, Default::default()),
                JavaMethodProto::new("setProgram", "(III)V", Self::set_program, Default::default()),
                JavaMethodProto::new("shortMidiEvent", "(III)V", Self::short_midi_event, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn bank_query(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }
    async fn get_channel_volume(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _channel: i32) -> Result<i32> {
        Ok(127)
    }
    async fn set_channel_volume(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _channel: i32, _volume: i32) -> Result<()> {
        Ok(())
    }
    async fn get_program(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _channel: i32,
    ) -> Result<ClassInstanceRef<jvm::Array<i32>>> {
        let array = jvm.instantiate_array("I", 2).await?;
        Ok(array.into())
    }
    async fn set_program(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _channel: i32, _bank: i32, _program: i32) -> Result<()> {
        Ok(())
    }
    async fn short_midi_event(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _type: i32, _data1: i32, _data2: i32) -> Result<()> {
        Ok(())
    }
}

impl StopTimeControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/StopTimeControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getStopTime", "()J", Self::get_stop_time, Default::default()),
                JavaMethodProto::new("setStopTime", "(J)V", Self::set_stop_time, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("stopTime", "J", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "stopTime", "J", -1i64).await
    }
    async fn get_stop_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "stopTime", "J").await
    }
    async fn set_stop_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        jvm.put_field(&mut this, "stopTime", "J", time).await
    }
}

impl MetaDataControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/MetaDataControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getKeys", "()[Ljava/lang/String;", Self::get_keys, Default::default()),
                JavaMethodProto::new(
                    "getKeyValue",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_key_value,
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
    async fn get_keys(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<jvm::Array<ClassInstanceRef<crate::classes::java::lang::String>>>> {
        let array = jvm.instantiate_array("Ljava/lang/String;", 0).await?;
        Ok(array.into())
    }
    async fn get_key_value(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _key: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::String>> {
        Ok(ClassInstanceRef::new(None))
    }
}

pub struct VideoControl;

impl VideoControl {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/control/VideoControl",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Control"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "initDisplayMode",
                    "(ILjava/lang/Object;)Ljava/lang/Object;",
                    Self::init_display_mode,
                    Default::default(),
                ),
                JavaMethodProto::new("setDisplaySize", "(II)V", Self::set_display_size, Default::default()),
                JavaMethodProto::new("setDisplayLocation", "(II)V", Self::set_display_location, Default::default()),
                JavaMethodProto::new("setDisplayFullScreen", "(Z)V", Self::set_display_full_screen, Default::default()),
                JavaMethodProto::new("setVisible", "(Z)V", Self::set_visible, Default::default()),
                JavaMethodProto::new("getSourceWidth", "()I", Self::get_source_width, Default::default()),
                JavaMethodProto::new("getSourceHeight", "()I", Self::get_source_height, Default::default()),
                JavaMethodProto::new("getSnapshot", "(Ljava/lang/String;)[B", Self::get_snapshot, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn init_display_mode(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _mode: i32,
        _arg: ClassInstanceRef<crate::classes::java::lang::Object>,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        let item = jvm.new_class("javax/microedition/lcdui/Item", "()V", ()).await?;
        Ok(item.into())
    }
    stub_void! {
        set_display_size(_width: i32, _height: i32);
        set_display_location(_x: i32, _y: i32);
        set_display_full_screen(_fullscreen: bool);
        set_visible(_visible: bool);
    }
    async fn get_source_width(_: &Jvm, context: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(context.screen_width())
    }
    async fn get_source_height(_: &Jvm, context: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(context.screen_height())
    }
    async fn get_snapshot(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _type: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<ClassInstanceRef<jvm::Array<i8>>> {
        Ok(jvm.instantiate_array("B", 0).await?.into())
    }
}
