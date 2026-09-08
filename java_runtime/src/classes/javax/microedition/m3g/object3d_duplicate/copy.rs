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
    pub(super) async fn copy_object3d_fields(jvm: &Jvm, source: &ClassInstanceRef<Object3D>, target: &mut ClassInstanceRef<Object3D>) -> Result<()> {
        let user_id = jvm.get_field(source, "userID", "I").await.unwrap_or(0);
        let user_object: ClassInstanceRef<Object> = jvm
            .get_field(source, "userObject", "Ljava/lang/Object;")
            .await
            .unwrap_or_else(|_| null_ref());
        let animation_tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(source, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(target, "userID", "I", user_id).await?;
        jvm.put_field(target, "userObject", "Ljava/lang/Object;", user_object).await?;
        jvm.put_field(target, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;", animation_tracks)
            .await
    }

    pub(super) async fn copy_transformable_fields(
        jvm: &Jvm,
        source: &ClassInstanceRef<Transformable>,
        target: &mut ClassInstanceRef<Transformable>,
    ) -> Result<()> {
        let source_object = cast_ref(source);
        let mut target_object = cast_ref(target);
        Self::copy_object3d_fields(jvm, &source_object, &mut target_object).await?;
        Self::copy_f32_field(jvm, source, target, "translationX", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "translationY", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "translationZ", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleX", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleY", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "scaleZ", 1.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationAngle", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationX", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationY", 0.0).await?;
        Self::copy_f32_field(jvm, source, target, "orientationZ", 1.0).await?;
        let matrix = Transformable::generic_matrix(jvm, source).await.unwrap_or_else(|_| identity_matrix());
        Transformable::put_generic_matrix(jvm, target, matrix).await
    }

    pub(super) async fn copy_node_fields(jvm: &Jvm, source: &ClassInstanceRef<Node>, target: &mut ClassInstanceRef<Node>) -> Result<()> {
        let source_transformable = cast_ref(source);
        let mut target_transformable = cast_ref(target);
        Self::copy_transformable_fields(jvm, &source_transformable, &mut target_transformable).await?;
        let rendering_enabled = jvm.get_field(source, "renderingEnabled", "Z").await.unwrap_or(true);
        let picking_enabled = jvm.get_field(source, "pickingEnabled", "Z").await.unwrap_or(true);
        let alpha_factor = jvm.get_field::<f32>(source, "alphaFactor", "F").await.unwrap_or(1.0);
        let scope = jvm.get_field(source, "scope", "I").await.unwrap_or(-1);
        let z_reference: ClassInstanceRef<Node> = jvm
            .get_field(source, "zReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let y_reference: ClassInstanceRef<Node> = jvm
            .get_field(source, "yReference", "Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        let z_target = jvm.get_field(source, "zTarget", "I").await.unwrap_or(Node::NONE);
        let y_target = jvm.get_field(source, "yTarget", "I").await.unwrap_or(Node::NONE);
        jvm.put_field(target, "parent", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(target, "renderingEnabled", "Z", rendering_enabled).await?;
        jvm.put_field(target, "pickingEnabled", "Z", picking_enabled).await?;
        jvm.put_field(target, "alphaFactor", "F", alpha_factor).await?;
        jvm.put_field(target, "scope", "I", scope).await?;
        jvm.put_field(target, "zReference", "Ljavax/microedition/m3g/Node;", z_reference).await?;
        jvm.put_field(target, "yReference", "Ljavax/microedition/m3g/Node;", y_reference).await?;
        jvm.put_field(target, "zTarget", "I", z_target).await?;
        jvm.put_field(target, "yTarget", "I", y_target).await
    }

    pub(super) async fn copy_group_fields(jvm: &Jvm, source: &ClassInstanceRef<Group>, target: &mut ClassInstanceRef<Group>) -> Result<()> {
        let source_node = cast_ref(source);
        let mut target_node = cast_ref(target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
        let children = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", 0).await?;
        jvm.put_field(target, "children", "[Ljavax/microedition/m3g/Node;", children).await?;
        jvm.put_field(target, "childCount", "I", 0).await
    }

    pub(super) async fn duplicate_group_children(
        jvm: &Jvm,
        source: &ClassInstanceRef<Group>,
        target: &mut ClassInstanceRef<Group>,
        map: &mut M3gDuplicateMap,
    ) -> Result<()> {
        let child_count: i32 = jvm.get_field(source, "childCount", "I").await.unwrap_or(0);
        if child_count <= 0 {
            return Ok(());
        }

        let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm.get_field(source, "children", "[Ljavax/microedition/m3g/Node;").await?;
        let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
        for child in children {
            if child.is_null() {
                continue;
            }
            let source_object = cast_ref(&child);
            let duplicate = Self::duplicate_object(jvm, &source_object, map).await?;
            Group::push_child(jvm, target, cast_ref::<Object3D, Node>(&duplicate)).await?;
        }
        Ok(())
    }

    pub(super) async fn duplicate_typed_ref<T>(jvm: &Jvm, source: ClassInstanceRef<T>, map: &mut M3gDuplicateMap) -> Result<ClassInstanceRef<T>> {
        if source.is_null() {
            return Ok(null_ref());
        }
        let source_object = cast_ref(&source);
        let duplicate = Self::duplicate_object(jvm, &source_object, map).await?;
        Ok(cast_ref(&duplicate))
    }

    pub(super) async fn new_object<T>(jvm: &Jvm, class_name: &str) -> Result<ClassInstanceRef<T>> {
        Ok(jvm.new_class(class_name, "()V", ()).await?.into())
    }
}
