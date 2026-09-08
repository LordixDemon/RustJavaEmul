use alloc::{string::String as RustString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{lang::String, util::Vector},
        javax::microedition::media::{Control, PlayerListener},
    },
};

// concrete runtime stand-in for interface javax.microedition.media.Player
pub struct Player;

impl Player {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/Player",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/media/Controllable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("realize", "()V", Self::realize, Default::default()),
                JavaMethodProto::new("prefetch", "()V", Self::prefetch, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("deallocate", "()V", Self::deallocate, Default::default()),
                JavaMethodProto::new(
                    "addPlayerListener",
                    "(Ljavax/microedition/media/PlayerListener;)V",
                    Self::add_player_listener,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "removePlayerListener",
                    "(Ljavax/microedition/media/PlayerListener;)V",
                    Self::remove_player_listener,
                    Default::default(),
                ),
                JavaMethodProto::new("getState", "()I", Self::get_state, Default::default()),
                JavaMethodProto::new("setLoopCount", "(I)V", Self::set_loop_count, Default::default()),
                JavaMethodProto::new("setMediaTime", "(J)J", Self::set_media_time, Default::default()),
                JavaMethodProto::new("getMediaTime", "()J", Self::get_media_time, Default::default()),
                JavaMethodProto::new("getDuration", "()J", Self::get_duration, Default::default()),
                JavaMethodProto::new("getContentType", "()Ljava/lang/String;", Self::get_content_type, Default::default()),
                JavaMethodProto::new(
                    "getControl",
                    "(Ljava/lang/String;)Ljavax/microedition/media/Control;",
                    Self::get_control,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getControls",
                    "()[Ljavax/microedition/media/Control;",
                    Self::get_controls,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("state", "I", Default::default()),
                JavaFieldProto::new("loopCount", "I", Default::default()),
                JavaFieldProto::new("mediaTime", "J", Default::default()),
                JavaFieldProto::new("UNREALIZED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("REALIZED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PREFETCHED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("STARTED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("CLOSED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("TIME_UNKNOWN", "J", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("listeners", "Ljava/util/Vector;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "state", "I", 100).await?;
        let listeners = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "listeners", "Ljava/util/Vector;", listeners).await?;

        Ok(())
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let class = "javax/microedition/media/Player";
        jvm.put_static_field(class, "UNREALIZED", "I", 100).await?;
        jvm.put_static_field(class, "REALIZED", "I", 200).await?;
        jvm.put_static_field(class, "PREFETCHED", "I", 300).await?;
        jvm.put_static_field(class, "STARTED", "I", 400).await?;
        jvm.put_static_field(class, "CLOSED", "I", 0).await?;
        jvm.put_static_field(class, "TIME_UNKNOWN", "J", -1i64).await
    }

    async fn realize(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::realize({this:?})");

        jvm.put_field(&mut this, "state", "I", 200).await?;
        Ok(())
    }

    async fn prefetch(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::prefetch({this:?})");

        jvm.put_field(&mut this, "state", "I", 300).await?;
        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::start({this:?})");

        let state: i32 = jvm.get_field(&this, "state", "I").await?;
        if state < 300 {
            Self::prefetch(jvm, context, this.clone()).await?;
        }
        jvm.put_field(&mut this, "state", "I", 400).await?;
        Self::fire_event(jvm, &this, "started").await?;
        Self::fire_event(jvm, &this, "endOfMedia").await?;
        jvm.put_field(&mut this, "state", "I", 300).await?;
        Ok(())
    }

    async fn stop(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::stop({this:?})");

        jvm.put_field(&mut this, "state", "I", 300).await?;
        Ok(())
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::close({this:?})");

        jvm.put_field(&mut this, "state", "I", 0).await?;
        Ok(())
    }

    async fn deallocate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::deallocate({this:?})");

        jvm.put_field(&mut this, "state", "I", 100).await?;
        Ok(())
    }

    async fn add_player_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<PlayerListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::addPlayerListener({this:?}, {listener:?})");
        if listener.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "listener").await);
        }
        let listeners: ClassInstanceRef<Vector> = jvm.get_field(&this, "listeners", "Ljava/util/Vector;").await?;
        let contains: bool = jvm
            .invoke_virtual(&listeners, "contains", "(Ljava/lang/Object;)Z", (listener.clone(),))
            .await?;
        if !contains {
            let _: () = jvm.invoke_virtual(&listeners, "addElement", "(Ljava/lang/Object;)V", (listener,)).await?;
        }
        Ok(())
    }

    async fn remove_player_listener(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<PlayerListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::removePlayerListener({this:?}, {listener:?})");
        if listener.is_null() {
            return Ok(());
        }
        let listeners: ClassInstanceRef<Vector> = jvm.get_field(&this, "listeners", "Ljava/util/Vector;").await?;
        let _: bool = jvm
            .invoke_virtual(&listeners, "removeElement", "(Ljava/lang/Object;)Z", (listener,))
            .await?;
        Ok(())
    }

    async fn fire_event(jvm: &Jvm, this: &ClassInstanceRef<Self>, event: &str) -> Result<()> {
        let listeners: ClassInstanceRef<Vector> = jvm.get_field(this, "listeners", "Ljava/util/Vector;").await?;
        if listeners.is_null() {
            return Ok(());
        }
        let event = JavaLangString::intern_rust_string(jvm, event).await?;
        let size: i32 = jvm.invoke_virtual(&listeners, "size", "()I", ()).await?;
        for index in 0..size {
            let listener: ClassInstanceRef<PlayerListener> = jvm.invoke_virtual(&listeners, "elementAt", "(I)Ljava/lang/Object;", (index,)).await?;
            if listener.is_null() {
                continue;
            }
            let result: Result<()> = jvm
                .invoke_virtual(
                    &listener,
                    "playerUpdate",
                    "(Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V",
                    (
                        this.clone(),
                        event.clone(),
                        ClassInstanceRef::<crate::classes::java::lang::Object>::new(None),
                    ),
                )
                .await;
            if let Err(JavaError::JavaException(exception)) = result {
                tracing::error!("Uncaught exception in PlayerListener.playerUpdate: {exception:?}");
            } else {
                result?;
            }
        }
        Ok(())
    }

    async fn get_state(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("javax.microedition.media.Player::getState({this:?})");

        jvm.get_field(&this, "state", "I").await
    }

    async fn set_loop_count(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, count: i32) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::setLoopCount({this:?}, {count:?})");

        jvm.put_field(&mut this, "loopCount", "I", count).await?;
        Ok(())
    }

    async fn set_media_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, now: i64) -> Result<i64> {
        tracing::debug!("javax.microedition.media.Player::setMediaTime({this:?}, {now:?})");

        jvm.put_field(&mut this, "mediaTime", "J", now).await?;
        Ok(now)
    }

    async fn get_media_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "mediaTime", "J").await
    }

    async fn get_duration(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(-1)
    }

    async fn get_content_type(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, "audio/midi").await?.into())
    }

    async fn get_control(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        control_type: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Control>> {
        let control_type = if control_type.is_null() {
            RustString::new()
        } else {
            JavaLangString::to_rust_string(jvm, &control_type).await?
        };
        tracing::debug!("javax.microedition.media.Player::getControl({this:?}, {control_type:?})");

        if control_type.contains("VolumeControl") || control_type.contains("Volume") {
            Ok(jvm.new_class("javax/microedition/media/control/VolumeControl", "()V", ()).await?.into())
        } else if control_type.contains("ToneControl") {
            Ok(jvm.new_class("javax/microedition/media/control/ToneControl", "()V", ()).await?.into())
        } else if control_type.contains("TempoControl") || control_type.contains("RateControl") {
            Ok(jvm.new_class("javax/microedition/media/control/TempoControl", "()V", ()).await?.into())
        } else if control_type.contains("PitchControl") {
            Ok(jvm.new_class("javax/microedition/media/control/PitchControl", "()V", ()).await?.into())
        } else if control_type.contains("MIDIControl") {
            Ok(jvm.new_class("javax/microedition/media/control/MIDIControl", "()V", ()).await?.into())
        } else if control_type.contains("StopTimeControl") {
            Ok(jvm.new_class("javax/microedition/media/control/StopTimeControl", "()V", ()).await?.into())
        } else if control_type.contains("MetaDataControl") {
            Ok(jvm.new_class("javax/microedition/media/control/MetaDataControl", "()V", ()).await?.into())
        } else if control_type.contains("VideoControl") {
            Ok(jvm.new_class("javax/microedition/media/control/VideoControl", "()V", ()).await?.into())
        } else {
            Ok(None.into())
        }
    }

    async fn get_controls(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<Control>>>> {
        let volume = Self::get_control(jvm, context, this, JavaLangString::from_rust_string(jvm, "VolumeControl").await?.into()).await?;
        let mut array = jvm.instantiate_array("Ljavax/microedition/media/Control;", 1).await?;
        jvm.store_array(&mut array, 0, alloc::vec![volume]).await?;
        Ok(array.into())
    }
}
