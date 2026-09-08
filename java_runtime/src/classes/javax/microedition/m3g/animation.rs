#[allow(unused_imports)]
use super::common::*;
#[allow(unused_imports)]
use super::math::*;
#[allow(unused_imports)]
use super::prelude::*;
#[allow(unused_imports)]
use super::raw_arrays::*;
#[allow(unused_imports)]
use super::render::*;
#[allow(unused_imports)]
use super::types::*;

impl AnimationController {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/AnimationController",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getActiveIntervalEnd", "()I", Self::get_active_interval_end, Default::default()),
                JavaMethodProto::new("getActiveIntervalStart", "()I", Self::get_active_interval_start, Default::default()),
                JavaMethodProto::new("getPosition", "(I)F", Self::get_position, Default::default()),
                JavaMethodProto::new("getRefWorldTime", "()I", Self::get_ref_world_time, Default::default()),
                JavaMethodProto::new("getSpeed", "()F", Self::get_speed, Default::default()),
                JavaMethodProto::new("getWeight", "()F", Self::get_weight, Default::default()),
                JavaMethodProto::new("setActiveInterval", "(II)V", Self::set_active_interval, Default::default()),
                JavaMethodProto::new("setPosition", "(FI)V", Self::set_position, Default::default()),
                JavaMethodProto::new("setSpeed", "(FI)V", Self::set_speed, Default::default()),
                JavaMethodProto::new("setWeight", "(F)V", Self::set_weight, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("speed", "F", Default::default()),
                JavaFieldProto::new("weight", "F", Default::default()),
                JavaFieldProto::new("activeIntervalStart", "I", Default::default()),
                JavaFieldProto::new("activeIntervalEnd", "I", Default::default()),
                JavaFieldProto::new("refSequenceTime", "F", Default::default()),
                JavaFieldProto::new("refWorldTime", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "speed", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "weight", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "activeIntervalStart", "I", 0).await?;
        jvm.put_field(&mut this, "activeIntervalEnd", "I", i32::MAX).await?;
        jvm.put_field(&mut this, "refSequenceTime", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", 0).await
    }

    pub(super) async fn get_active_interval_end(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "activeIntervalEnd", "I").await
    }

    pub(super) async fn get_active_interval_start(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "activeIntervalStart", "I").await
    }

    pub(super) async fn get_position(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, world_time: i32) -> Result<f32> {
        Self::sequence_time(jvm, &this, world_time).await
    }

    pub(super) async fn get_ref_world_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "refWorldTime", "I").await
    }

    pub(super) async fn get_speed(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "speed", "F").await
    }

    pub(super) async fn get_weight(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "weight", "F").await
    }

    pub(super) async fn set_active_interval(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<()> {
        if end < start {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "AnimationController active interval")
                .await);
        }
        jvm.put_field(&mut this, "activeIntervalStart", "I", start).await?;
        jvm.put_field(&mut this, "activeIntervalEnd", "I", end).await
    }

    pub(super) async fn set_position(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence_time: f32,
        world_time: i32,
    ) -> Result<()> {
        jvm.put_field(&mut this, "refSequenceTime", "F", sequence_time).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", world_time).await
    }

    pub(super) async fn set_speed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, speed: f32, world_time: i32) -> Result<()> {
        let sequence_time = Self::sequence_time(jvm, &this, world_time).await.unwrap_or(0.0);
        jvm.put_field(&mut this, "speed", "F", speed).await?;
        jvm.put_field(&mut this, "refSequenceTime", "F", sequence_time).await?;
        jvm.put_field(&mut this, "refWorldTime", "I", world_time).await
    }

    pub(super) async fn set_weight(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, weight: f32) -> Result<()> {
        if weight < 0.0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "AnimationController weight").await);
        }
        jvm.put_field(&mut this, "weight", "F", weight).await
    }

    pub(super) async fn is_active(jvm: &Jvm, this: &ClassInstanceRef<Self>, world_time: i32) -> Result<bool> {
        if this.is_null() {
            return Ok(false);
        }
        let start: i32 = jvm.get_field(this, "activeIntervalStart", "I").await.unwrap_or(0);
        let end: i32 = jvm.get_field(this, "activeIntervalEnd", "I").await.unwrap_or(i32::MAX);
        let weight: f32 = jvm.get_field(this, "weight", "F").await.unwrap_or(1.0);
        if weight <= 0.0 {
            return Ok(false);
        }
        if start == 0 && end == 0 {
            return Ok(true);
        }
        Ok(world_time >= start && world_time < end)
    }

    pub(super) async fn sequence_time(jvm: &Jvm, this: &ClassInstanceRef<Self>, world_time: i32) -> Result<f32> {
        if this.is_null() {
            return Ok(world_time as f32);
        }
        let ref_sequence_time: f32 = jvm.get_field(this, "refSequenceTime", "F").await.unwrap_or(0.0);
        let ref_world_time: i32 = jvm.get_field(this, "refWorldTime", "I").await.unwrap_or(0);
        let speed: f32 = jvm.get_field(this, "speed", "F").await.unwrap_or(1.0);
        Ok(ref_sequence_time + (world_time - ref_world_time) as f32 * speed)
    }
}
