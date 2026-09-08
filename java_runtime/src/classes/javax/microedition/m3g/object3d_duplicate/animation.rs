#[allow(unused_imports)]
use super::super::common::*;
#[allow(unused_imports)]
use super::super::math::*;
#[allow(unused_imports)]
use super::super::prelude::*;
#[allow(unused_imports)]
use super::super::raw_arrays::*;
#[allow(unused_imports)]
use super::super::render::*;
#[allow(unused_imports)]
use super::super::types::*;

impl Object3D {
    pub(super) async fn duplicate_animation_controller(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<AnimationController> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_controller: ClassInstanceRef<AnimationController> = cast_ref(source);
        Self::copy_f32_field(jvm, &source_controller, &mut target, "speed", 1.0).await?;
        Self::copy_f32_field(jvm, &source_controller, &mut target, "weight", 1.0).await?;
        Self::copy_i32_field(jvm, &source_controller, &mut target, "activeIntervalStart", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_controller, &mut target, "activeIntervalEnd", "I", i32::MAX).await?;
        Self::copy_f32_field(jvm, &source_controller, &mut target, "refSequenceTime", 0.0).await?;
        Self::copy_i32_field(jvm, &source_controller, &mut target, "refWorldTime", "I", 0).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_animation_track(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<AnimationTrack> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_track: ClassInstanceRef<AnimationTrack> = cast_ref(source);
        let sequence: ClassInstanceRef<KeyframeSequence> = jvm
            .get_field(&source_track, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;")
            .await
            .unwrap_or_else(|_| null_ref());
        let controller: ClassInstanceRef<AnimationController> = jvm
            .get_field(&source_track, "controller", "Ljavax/microedition/m3g/AnimationController;")
            .await
            .unwrap_or_else(|_| null_ref());
        Self::copy_i32_field(jvm, &source_track, &mut target, "targetProperty", "I", 0).await?;
        jvm.put_field(
            &mut target,
            "sequence",
            "Ljavax/microedition/m3g/KeyframeSequence;",
            Self::duplicate_typed_ref(jvm, sequence, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "controller",
            "Ljavax/microedition/m3g/AnimationController;",
            Self::duplicate_typed_ref(jvm, controller, map).await?,
        )
        .await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_keyframe_sequence(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let source_sequence: ClassInstanceRef<KeyframeSequence> = cast_ref(source);
        let keyframe_count = jvm.get_field(&source_sequence, "keyframeCount", "I").await.unwrap_or(1).max(1);
        let component_count = jvm.get_field(&source_sequence, "componentCount", "I").await.unwrap_or(1).max(1);
        let interpolation = jvm
            .get_field(&source_sequence, "interpolationType", "I")
            .await
            .unwrap_or(KeyframeSequence::LINEAR);
        let mut target: ClassInstanceRef<KeyframeSequence> = jvm
            .new_class(&class_name, "(III)V", (keyframe_count, component_count, interpolation))
            .await?
            .into();
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        Self::copy_i32_field(jvm, &source_sequence, &mut target, "repeatMode", "I", KeyframeSequence::CONSTANT).await?;
        Self::copy_i32_field(jvm, &source_sequence, &mut target, "duration", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_sequence, &mut target, "validRangeFirst", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_sequence, &mut target, "validRangeLast", "I", 0).await?;
        let times: ClassInstanceRef<Array<i32>> = jvm
            .get_field(&source_sequence, "keyframeTimes", "[I")
            .await
            .unwrap_or_else(|_| null_ref());
        let values: ClassInstanceRef<Array<f32>> = jvm
            .get_field(&source_sequence, "keyframeValues", "[F")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "keyframeTimes", "[I", times).await?;
        jvm.put_field(&mut target, "keyframeValues", "[F", values).await?;
        Ok(duplicate)
    }
}
