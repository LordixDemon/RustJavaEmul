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
    pub(super) async fn animate(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, world_time: i32) -> Result<i32> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.animate").await);
        }

        let mut remaining = i32::MAX;
        let mut any = false;
        let mut stack = vec![this];
        let mut visited = Vec::new();
        while let Some(current) = stack.pop() {
            if current.is_null() || visited.iter().any(|seen| same_instance(seen, &current)) {
                continue;
            }
            visited.push(current.clone());
            if let Some(object_remaining) = Self::animate_one(jvm, &current, world_time).await? {
                any = true;
                remaining = remaining.min(object_remaining);
            }
            Self::push_find_references(jvm, &current, &mut stack).await?;
        }

        Ok(if any { remaining } else { 0 })
    }

    pub(super) async fn animate_one(jvm: &Jvm, object: &ClassInstanceRef<Self>, world_time: i32) -> Result<Option<i32>> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(object, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await
            .unwrap_or_else(|_| null_ref());
        if tracks.is_null() {
            return Ok(None);
        }

        let count = jvm.array_length(&tracks).await?;
        if count == 0 {
            return Ok(None);
        }

        let tracks: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
        let mut remaining = i32::MAX;
        let mut any = false;
        let mut blends: Vec<(i32, Vec<f32>, f32)> = Vec::new();
        for track in tracks {
            if track.is_null() {
                continue;
            }
            let controller: ClassInstanceRef<AnimationController> = jvm
                .get_field(&track, "controller", "Ljavax/microedition/m3g/AnimationController;")
                .await
                .unwrap_or_else(|_| null_ref());
            if !AnimationController::is_active(jvm, &controller, world_time).await? {
                continue;
            }

            any = true;
            let start: i32 = jvm.get_field(&controller, "activeIntervalStart", "I").await.unwrap_or(0);
            let end: i32 = jvm.get_field(&controller, "activeIntervalEnd", "I").await.unwrap_or(i32::MAX);
            if !(start == 0 && end == 0) {
                remaining = remaining.min(end.saturating_sub(world_time).max(0));
            }
            let weight: f32 = jvm.get_field::<f32>(&controller, "weight", "F").await.unwrap_or(1.0).max(0.0);
            if weight <= 0.0 {
                continue;
            }

            let sequence_time = AnimationController::sequence_time(jvm, &controller, world_time).await?;
            let sequence: ClassInstanceRef<KeyframeSequence> = jvm
                .get_field(&track, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;")
                .await
                .unwrap_or_else(|_| null_ref());
            let Some(values) = KeyframeSequence::sample(jvm, &sequence, sequence_time).await? else {
                continue;
            };
            let property: i32 = jvm.get_field(&track, "targetProperty", "I").await.unwrap_or(0);
            if let Some((_, accum, total)) = blends.iter_mut().find(|(candidate, _, _)| *candidate == property) {
                if accum.len() < values.len() {
                    accum.resize(values.len(), 0.0);
                }
                for (index, value) in values.iter().enumerate() {
                    accum[index] += *value * weight;
                }
                *total += weight;
            } else {
                blends.push((property, values.iter().map(|value| *value * weight).collect(), weight));
            }
        }

        for (property, mut values, weight) in blends {
            if weight >= 1.0 {
                if weight > 0.0 {
                    for value in &mut values {
                        *value /= weight;
                    }
                }
            } else {
                Self::mix_rest_pose(jvm, object, property, &mut values, weight).await?;
            }
            Self::apply_animation_values(jvm, object, property, &values).await?;
        }
        Ok(any.then_some(remaining))
    }

    pub(super) async fn mix_rest_pose(jvm: &Jvm, object: &ClassInstanceRef<Self>, property: i32, values: &mut [f32], weight: f32) -> Result<()> {
        let rest = Self::rest_pose_values(jvm, object, property, values.len()).await?;
        if rest.is_empty() {
            return Ok(());
        }
        let rest_weight = 1.0 - weight;
        match property {
            AnimationTrack::ORIENTATION if values.len() >= 4 && rest.len() >= 4 => {
                let mixed = nlerp_quaternion([rest[0], rest[1], rest[2], rest[3]], [values[0], values[1], values[2], values[3]], weight);
                values[0] = mixed[0];
                values[1] = mixed[1];
                values[2] = mixed[2];
                values[3] = mixed[3];
            }
            _ => {
                for (index, value) in values.iter_mut().enumerate() {
                    let rest_value = rest.get(index).copied().unwrap_or(0.0);
                    *value += rest_value * rest_weight;
                }
            }
        }
        Ok(())
    }

    pub(super) async fn rest_pose_values(jvm: &Jvm, object: &ClassInstanceRef<Self>, property: i32, len: usize) -> Result<Vec<f32>> {
        Ok(match property {
            AnimationTrack::ALPHA if Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                vec![
                    jvm.get_field::<f32>(&cast_ref::<Object3D, Node>(object), "alphaFactor", "F")
                        .await
                        .unwrap_or(1.0),
                ]
            }
            AnimationTrack::ORIENTATION if Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let transformable = cast_ref::<Object3D, Transformable>(object);
                let angle = jvm.get_field::<f32>(&transformable, "orientationAngle", "F").await.unwrap_or(0.0);
                let ax = jvm.get_field::<f32>(&transformable, "orientationX", "F").await.unwrap_or(0.0);
                let ay = jvm.get_field::<f32>(&transformable, "orientationY", "F").await.unwrap_or(0.0);
                let az = jvm.get_field::<f32>(&transformable, "orientationZ", "F").await.unwrap_or(1.0);
                axis_angle_to_quaternion(angle, ax, ay, az).to_vec()
            }
            AnimationTrack::PICKABILITY if Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                vec![i32::from(
                    jvm.get_field::<bool>(&cast_ref::<Object3D, Node>(object), "pickingEnabled", "Z")
                        .await
                        .unwrap_or(true),
                ) as f32]
            }
            AnimationTrack::SCALE if Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let transformable = cast_ref::<Object3D, Transformable>(object);
                vec![
                    jvm.get_field::<f32>(&transformable, "scaleX", "F").await.unwrap_or(1.0),
                    jvm.get_field::<f32>(&transformable, "scaleY", "F").await.unwrap_or(1.0),
                    jvm.get_field::<f32>(&transformable, "scaleZ", "F").await.unwrap_or(1.0),
                ]
            }
            AnimationTrack::TRANSLATION if Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let transformable = cast_ref::<Object3D, Transformable>(object);
                vec![
                    jvm.get_field::<f32>(&transformable, "translationX", "F").await.unwrap_or(0.0),
                    jvm.get_field::<f32>(&transformable, "translationY", "F").await.unwrap_or(0.0),
                    jvm.get_field::<f32>(&transformable, "translationZ", "F").await.unwrap_or(0.0),
                ]
            }
            AnimationTrack::VISIBILITY if Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                vec![i32::from(
                    jvm.get_field::<bool>(&cast_ref::<Object3D, Node>(object), "renderingEnabled", "Z")
                        .await
                        .unwrap_or(true),
                ) as f32]
            }
            AnimationTrack::MORPH_WEIGHTS if Self::is_class(jvm, object, "javax/microedition/m3g/MorphingMesh") => {
                let weights: ClassInstanceRef<Array<f32>> = jvm
                    .get_field(&cast_ref::<Object3D, MorphingMesh>(object), "weights", "[F")
                    .await
                    .unwrap_or_else(|_| null_ref());
                if weights.is_null() {
                    vec![0.0; len]
                } else {
                    let count = jvm.array_length(&weights).await?.min(len);
                    let mut values: Vec<f32> = jvm.load_array(&weights, 0, count).await?;
                    values.resize(len, 0.0);
                    values
                }
            }
            _ => Vec::new(),
        })
    }

    pub(super) fn color_from_values(values: &[f32], existing: i32) -> i32 {
        let r = (values.first().copied().unwrap_or(0.0).clamp(0.0, 1.0) * 255.0) as u32;
        let g = (values.get(1).copied().unwrap_or(0.0).clamp(0.0, 1.0) * 255.0) as u32;
        let b = (values.get(2).copied().unwrap_or(0.0).clamp(0.0, 1.0) * 255.0) as u32;
        let a = if values.len() >= 4 {
            (values[3].clamp(0.0, 1.0) * 255.0) as u32
        } else {
            ((existing as u32) >> 24) & 0xff
        };
        ((a << 24) | (r << 16) | (g << 8) | b) as i32
    }

    pub(super) async fn apply_animation_values(jvm: &Jvm, object: &ClassInstanceRef<Self>, property: i32, values: &[f32]) -> Result<()> {
        match property {
            AnimationTrack::ALPHA if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                let mut node = cast_ref::<Object3D, Node>(object);
                jvm.put_field(&mut node, "alphaFactor", "F", values[0].clamp(0.0, 1.0)).await?;
            }
            AnimationTrack::AMBIENT_COLOR if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Material") => {
                let mut material = cast_ref::<Object3D, Material>(object);
                let existing = jvm.get_field(&material, "ambientColor", "I").await.unwrap_or(0);
                jvm.put_field(&mut material, "ambientColor", "I", Self::color_from_values(values, existing))
                    .await?;
            }
            AnimationTrack::COLOR if values.len() >= 3 => {
                if Self::is_class(jvm, object, "javax/microedition/m3g/Light") {
                    let mut light = cast_ref::<Object3D, Light>(object);
                    let existing = jvm.get_field(&light, "color", "I").await.unwrap_or(0);
                    jvm.put_field(&mut light, "color", "I", Self::color_from_values(values, existing) & 0x00ff_ffff)
                        .await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Fog") {
                    let mut fog = cast_ref::<Object3D, Fog>(object);
                    let existing = jvm.get_field(&fog, "color", "I").await.unwrap_or(0);
                    jvm.put_field(&mut fog, "color", "I", Self::color_from_values(values, existing)).await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Background") {
                    let mut background = cast_ref::<Object3D, Background>(object);
                    let existing = jvm.get_field(&background, "color", "I").await.unwrap_or(0);
                    jvm.put_field(&mut background, "color", "I", Self::color_from_values(values, existing))
                        .await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/VertexBuffer") {
                    let mut buffer = cast_ref::<Object3D, VertexBuffer>(object);
                    let existing = jvm.get_field(&buffer, "defaultColor", "I").await.unwrap_or(0);
                    jvm.put_field(&mut buffer, "defaultColor", "I", Self::color_from_values(values, existing))
                        .await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Texture2D") {
                    let mut texture = cast_ref::<Object3D, Texture2D>(object);
                    let existing = jvm.get_field(&texture, "blendColor", "I").await.unwrap_or(0);
                    jvm.put_field(&mut texture, "blendColor", "I", Self::color_from_values(values, existing) & 0x00ff_ffff)
                        .await?;
                }
            }
            AnimationTrack::CROP if values.len() >= 4 => {
                if Self::is_class(jvm, object, "javax/microedition/m3g/Sprite3D") {
                    let mut sprite = cast_ref::<Object3D, Sprite3D>(object);
                    jvm.put_field(&mut sprite, "cropX", "I", values[0] as i32).await?;
                    jvm.put_field(&mut sprite, "cropY", "I", values[1] as i32).await?;
                    jvm.put_field(&mut sprite, "cropW", "I", values[2] as i32).await?;
                    jvm.put_field(&mut sprite, "cropH", "I", values[3] as i32).await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Background") {
                    let mut background = cast_ref::<Object3D, Background>(object);
                    jvm.put_field(&mut background, "cropX", "I", values[0] as i32).await?;
                    jvm.put_field(&mut background, "cropY", "I", values[1] as i32).await?;
                    jvm.put_field(&mut background, "cropW", "I", values[2] as i32).await?;
                    jvm.put_field(&mut background, "cropH", "I", values[3] as i32).await?;
                }
            }
            AnimationTrack::DENSITY if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Fog") => {
                let mut fog = cast_ref::<Object3D, Fog>(object);
                jvm.put_field(&mut fog, "density", "F", values[0].max(0.0)).await?;
            }
            AnimationTrack::DIFFUSE_COLOR if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Material") => {
                let mut material = cast_ref::<Object3D, Material>(object);
                let existing = jvm.get_field(&material, "diffuseColor", "I").await.unwrap_or(0);
                jvm.put_field(&mut material, "diffuseColor", "I", Self::color_from_values(values, existing))
                    .await?;
            }
            AnimationTrack::EMISSIVE_COLOR if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Material") => {
                let mut material = cast_ref::<Object3D, Material>(object);
                let existing = jvm.get_field(&material, "emissiveColor", "I").await.unwrap_or(0);
                jvm.put_field(&mut material, "emissiveColor", "I", Self::color_from_values(values, existing))
                    .await?;
            }
            AnimationTrack::FAR_DISTANCE if !values.is_empty() => {
                if Self::is_class(jvm, object, "javax/microedition/m3g/Camera") {
                    let mut camera = cast_ref::<Object3D, Camera>(object);
                    jvm.put_field(&mut camera, "far", "F", values[0]).await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Fog") {
                    let mut fog = cast_ref::<Object3D, Fog>(object);
                    jvm.put_field(&mut fog, "far", "F", values[0]).await?;
                }
            }
            AnimationTrack::FIELD_OF_VIEW if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Camera") => {
                let mut camera = cast_ref::<Object3D, Camera>(object);
                let mode = Camera::projection_mode(jvm, &camera).await.unwrap_or(Camera::PERSPECTIVE);
                if mode == Camera::PARALLEL {
                    jvm.put_field(&mut camera, "parallelHeight", "F", values[0].abs().max(0.0001)).await?;
                } else {
                    jvm.put_field(&mut camera, "fovy", "F", values[0]).await?;
                }
            }
            AnimationTrack::INTENSITY if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Light") => {
                let mut light = cast_ref::<Object3D, Light>(object);
                jvm.put_field(&mut light, "intensity", "F", values[0]).await?;
            }
            AnimationTrack::MORPH_WEIGHTS if Self::is_class(jvm, object, "javax/microedition/m3g/MorphingMesh") => {
                let mut mesh = cast_ref::<Object3D, MorphingMesh>(object);
                let mut weights = jvm.instantiate_array("F", values.len()).await?;
                jvm.store_array(&mut weights, 0, values.to_vec()).await?;
                jvm.put_field(&mut mesh, "weights", "[F", weights).await?;
            }
            AnimationTrack::NEAR_DISTANCE if !values.is_empty() => {
                if Self::is_class(jvm, object, "javax/microedition/m3g/Camera") {
                    let mut camera = cast_ref::<Object3D, Camera>(object);
                    jvm.put_field(&mut camera, "near", "F", values[0]).await?;
                } else if Self::is_class(jvm, object, "javax/microedition/m3g/Fog") {
                    let mut fog = cast_ref::<Object3D, Fog>(object);
                    jvm.put_field(&mut fog, "near", "F", values[0]).await?;
                }
            }
            AnimationTrack::ORIENTATION if values.len() >= 4 && Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let mut transformable = cast_ref::<Object3D, Transformable>(object);
                let (angle, ax, ay, az) = quaternion_to_axis_angle(values[0], values[1], values[2], values[3]);
                Transformable::set_orientation_fields(jvm, &mut transformable, angle, ax, ay, az).await?;
            }
            AnimationTrack::PICKABILITY if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                let mut node = cast_ref::<Object3D, Node>(object);
                jvm.put_field(&mut node, "pickingEnabled", "Z", values[0] >= 0.5).await?;
            }
            AnimationTrack::SCALE if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let mut transformable = cast_ref::<Object3D, Transformable>(object);
                Transformable::set_scale_fields(jvm, &mut transformable, values[0], values[1], values[2]).await?;
            }
            AnimationTrack::SHININESS if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Material") => {
                let mut material = cast_ref::<Object3D, Material>(object);
                jvm.put_field(&mut material, "shininess", "F", values[0].clamp(0.0, 128.0)).await?;
            }
            AnimationTrack::SPECULAR_COLOR if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Material") => {
                let mut material = cast_ref::<Object3D, Material>(object);
                let existing = jvm.get_field(&material, "specularColor", "I").await.unwrap_or(0);
                jvm.put_field(&mut material, "specularColor", "I", Self::color_from_values(values, existing))
                    .await?;
            }
            AnimationTrack::SPOT_ANGLE if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Light") => {
                let mut light = cast_ref::<Object3D, Light>(object);
                jvm.put_field(&mut light, "spotAngle", "F", values[0].clamp(0.0, 90.0)).await?;
            }
            AnimationTrack::SPOT_EXPONENT if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Light") => {
                let mut light = cast_ref::<Object3D, Light>(object);
                jvm.put_field(&mut light, "spotExponent", "F", values[0].clamp(0.0, 128.0)).await?;
            }
            AnimationTrack::TRANSLATION if values.len() >= 3 && Self::is_class(jvm, object, "javax/microedition/m3g/Transformable") => {
                let mut transformable = cast_ref::<Object3D, Transformable>(object);
                Transformable::set_translation_fields(jvm, &mut transformable, values[0], values[1], values[2]).await?;
            }
            AnimationTrack::VISIBILITY if !values.is_empty() && Self::is_class(jvm, object, "javax/microedition/m3g/Node") => {
                let mut node = cast_ref::<Object3D, Node>(object);
                jvm.put_field(&mut node, "renderingEnabled", "Z", values[0] >= 0.5).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
