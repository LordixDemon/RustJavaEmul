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

mod animate;

impl Object3D {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Object3D",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addAnimationTrack",
                    "(Ljavax/microedition/m3g/AnimationTrack;)V",
                    Self::add_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("animate", "(I)I", Self::animate, Default::default()),
                JavaMethodProto::new("duplicate", "()Ljavax/microedition/m3g/Object3D;", Self::duplicate, Default::default()),
                JavaMethodProto::new("find", "(I)Ljavax/microedition/m3g/Object3D;", Self::find, Default::default()),
                JavaMethodProto::new(
                    "getAnimationTrack",
                    "(I)Ljavax/microedition/m3g/AnimationTrack;",
                    Self::get_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("getAnimationTrackCount", "()I", Self::get_animation_track_count, Default::default()),
                JavaMethodProto::new("getUserID", "()I", Self::get_user_id, Default::default()),
                JavaMethodProto::new(
                    "removeAnimationTrack",
                    "(Ljavax/microedition/m3g/AnimationTrack;)V",
                    Self::remove_animation_track,
                    Default::default(),
                ),
                JavaMethodProto::new("setUserID", "(I)V", Self::set_user_id, Default::default()),
                JavaMethodProto::new("setUserObject", "(Ljava/lang/Object;)V", Self::set_user_object, Default::default()),
                JavaMethodProto::new("getUserObject", "()Ljava/lang/Object;", Self::get_user_object, Default::default()),
                JavaMethodProto::new(
                    "getReferences",
                    "([Ljavax/microedition/m3g/Object3D;)I",
                    Self::get_references,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("userID", "I", Default::default()),
                JavaFieldProto::new("userObject", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "userID", "I", 0).await?;
        jvm.put_field(&mut this, "userObject", "Ljava/lang/Object;", null_ref::<Object>()).await?;
        let animation_tracks = jvm.instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", 0).await?;
        jvm.put_field(&mut this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", animation_tracks)
            .await
    }

    pub(super) async fn duplicate(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.duplicate").await);
        }

        let class_name = this.class_definition().name().to_string();
        let mut map = M3gDuplicateMap::default();
        let duplicate = Self::duplicate_object(jvm, &this, &mut map).await?;
        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Object3D.duplicate class={} copied={} shared={}",
            class_name,
            map.copied,
            map.shared
        );
        Ok(duplicate)
    }

    pub(super) async fn find(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, user_id: i32) -> Result<ClassInstanceRef<Self>> {
        let mut stack = vec![this];
        let mut visited = Vec::new();
        while let Some(current) = stack.pop() {
            if current.is_null() {
                continue;
            }
            if visited.iter().any(|seen| same_instance(seen, &current)) {
                continue;
            }
            visited.push(current.clone());

            let current_id: i32 = jvm.get_field(&current, "userID", "I").await?;
            if current_id == user_id {
                tracing::debug!(
                    target: "rustjava_m3g",
                    "m3g.Object3D.find userID={} hit class={} visited={}",
                    user_id,
                    current.class_definition().name(),
                    visited.len()
                );
                return Ok(current);
            }

            Self::push_find_references(jvm, &current, &mut stack).await?;
        }

        tracing::debug!(
            target: "rustjava_m3g",
            "m3g.Object3D.find userID={} miss visited={}",
            user_id,
            visited.len()
        );
        Ok(null_ref())
    }

    pub(super) async fn add_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        track: ClassInstanceRef<AnimationTrack>,
    ) -> Result<()> {
        if track.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.addAnimationTrack").await);
        }
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        let mut values: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        if values.iter().any(|candidate| same_instance(candidate, &track)) {
            return Ok(());
        }
        values.push(track);
        Self::store_animation_tracks(jvm, &mut this, values).await
    }

    pub(super) fn is_class(jvm: &Jvm, object: &ClassInstanceRef<Self>, name: &str) -> bool {
        object.instance.as_deref().is_some_and(|instance| jvm.is_instance(instance, name))
    }

    pub(super) async fn get_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
    ) -> Result<ClassInstanceRef<AnimationTrack>> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        if index < 0 || index as usize >= count {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", "Object3D animation track index")
                .await);
        }
        Ok(jvm
            .load_array(&tracks, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap_or_else(null_ref))
    }

    pub(super) async fn get_animation_track_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        Ok(jvm.array_length(&tracks).await? as i32)
    }

    pub(super) async fn remove_animation_track(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        track: ClassInstanceRef<AnimationTrack>,
    ) -> Result<()> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(&this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await?;
        let count = jvm.array_length(&tracks).await?;
        let mut values: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        values.retain(|candidate| !same_instance(candidate, &track));
        Self::store_animation_tracks(jvm, &mut this, values).await
    }

    pub(super) async fn store_animation_tracks(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        values: Vec<ClassInstanceRef<AnimationTrack>>,
    ) -> Result<()> {
        let mut array = jvm.instantiate_array("Ljavax/microedition/m3g/AnimationTrack;", values.len()).await?;
        jvm.store_array(&mut array, 0, values).await?;
        jvm.put_field(this, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", array)
            .await
    }
}
