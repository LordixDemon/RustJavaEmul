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

mod animation;
mod appearance;
mod copy;
mod fields;
mod geometry;
mod nodes;
mod references;

impl Object3D {
    #[async_recursion::async_recursion]
    pub(super) async fn duplicate_object(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
    ) -> Result<ClassInstanceRef<Object3D>> {
        if source.is_null() {
            return Ok(null_ref());
        }
        if let Some(duplicate) = map.get(source) {
            return Ok(duplicate);
        }

        let class_name = source.class_definition().name().to_string();
        match class_name.as_str() {
            "javax/microedition/m3g/World" => Self::duplicate_world(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Group" => Self::duplicate_group(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Mesh" | "javax/microedition/m3g/SkinnedMesh" | "javax/microedition/m3g/MorphingMesh" => {
                Self::duplicate_mesh(jvm, source, map, &class_name).await
            }
            "javax/microedition/m3g/Appearance" => Self::duplicate_appearance(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Camera" => Self::duplicate_camera(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Background" => Self::duplicate_background(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/CompositingMode" => Self::duplicate_compositing_mode(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Fog" => Self::duplicate_fog(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/PolygonMode" => Self::duplicate_polygon_mode(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Texture2D" => Self::duplicate_texture2d(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Sprite3D" => Self::duplicate_sprite3d(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Image2D" => Self::duplicate_image2d(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Light" => Self::duplicate_light(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/Material" | "javax/microedition/m3g/IndexBuffer" => {
                Self::duplicate_material_or_index_buffer(jvm, source, map, &class_name).await
            }
            "javax/microedition/m3g/TriangleStripArray" => Self::duplicate_triangle_strip_array(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/VertexArray" => Self::duplicate_vertex_array(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/VertexBuffer" => Self::duplicate_vertex_buffer(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/AnimationController" => Self::duplicate_animation_controller(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/AnimationTrack" => Self::duplicate_animation_track(jvm, source, map, &class_name).await,
            "javax/microedition/m3g/KeyframeSequence" => Self::duplicate_keyframe_sequence(jvm, source, map, &class_name).await,
            _ => {
                tracing::debug!(target: "rustjava_m3g", "m3g.Object3D.duplicate share unsupported class={class_name}");
                map.entries.push((source.clone(), source.clone()));
                map.note_shared();
                Ok(source.clone())
            }
        }
    }
}
