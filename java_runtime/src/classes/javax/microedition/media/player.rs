use alloc::{string::String as RustString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::lang::String,
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
                JavaMethodProto::new(
                    "getControl",
                    "(Ljava/lang/String;)Ljavax/microedition/media/Control;",
                    Self::get_control,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("state", "I", Default::default()),
                JavaFieldProto::new("loopCount", "I", Default::default()),
                JavaFieldProto::new("mediaTime", "J", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "state", "I", 100).await?;

        Ok(())
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

    async fn start(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::start({this:?})");

        jvm.put_field(&mut this, "state", "I", 400).await?;
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
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<PlayerListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::addPlayerListener({this:?}, {listener:?})");
        Ok(())
    }

    async fn remove_player_listener(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        listener: ClassInstanceRef<PlayerListener>,
    ) -> Result<()> {
        tracing::debug!("javax.microedition.media.Player::removePlayerListener({this:?}, {listener:?})");
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
        } else {
            Ok(None.into())
        }
    }
}
