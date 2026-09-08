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
    pub(super) async fn duplicate_world(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<World> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);

        let source_group = cast_ref(source);
        let mut target_group = cast_ref(&target);
        Self::copy_group_fields(jvm, &source_group, &mut target_group).await?;
        Self::duplicate_group_children(jvm, &source_group, &mut target_group, map).await?;

        let camera: ClassInstanceRef<Camera> = jvm
            .get_field(&cast_ref::<Object3D, World>(source), "activeCamera", "Ljavax/microedition/m3g/Camera;")
            .await
            .unwrap_or_else(|_| null_ref());
        let background: ClassInstanceRef<Background> = jvm
            .get_field(&cast_ref::<Object3D, World>(source), "background", "Ljavax/microedition/m3g/Background;")
            .await
            .unwrap_or_else(|_| null_ref());
        let camera = Self::duplicate_typed_ref(jvm, camera, map).await?;
        let background = Self::duplicate_typed_ref(jvm, background, map).await?;
        jvm.put_field(&mut target, "activeCamera", "Ljavax/microedition/m3g/Camera;", camera)
            .await?;
        jvm.put_field(&mut target, "background", "Ljavax/microedition/m3g/Background;", background)
            .await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_group(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Group> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_group = cast_ref(source);
        Self::copy_group_fields(jvm, &source_group, &mut target).await?;
        Self::duplicate_group_children(jvm, &source_group, &mut target, map).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_mesh(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Mesh> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_mesh: ClassInstanceRef<Mesh> = cast_ref(source);
        let source_node = cast_ref(&source_mesh);
        let mut target_node = cast_ref(&target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;

        let vertex_buffer: ClassInstanceRef<VertexBuffer> = jvm
            .get_field(&source_mesh, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;")
            .await
            .unwrap_or_else(|_| null_ref());
        let submesh_count: i32 = jvm.get_field(&source_mesh, "submeshCount", "I").await.unwrap_or(0);
        let count = submesh_count.max(0) as usize;

        let mut index_array = jvm.instantiate_array("Ljavax/microedition/m3g/IndexBuffer;", count).await?;
        let source_indices: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> = jvm
            .get_field(&source_mesh, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !source_indices.is_null() && count > 0 {
            let indices: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&source_indices, 0, count).await?;
            jvm.store_array(&mut index_array, 0, indices).await?;
        }

        let mut appearance_array = jvm.instantiate_array("Ljavax/microedition/m3g/Appearance;", count).await?;
        let source_appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> = jvm
            .get_field(&source_mesh, "appearances", "[Ljavax/microedition/m3g/Appearance;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !source_appearances.is_null() && count > 0 {
            let appearances: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&source_appearances, 0, count).await?;
            let mut duplicates = Vec::with_capacity(appearances.len());
            for appearance in appearances {
                duplicates.push(Self::duplicate_typed_ref(jvm, appearance, map).await?);
            }
            jvm.store_array(&mut appearance_array, 0, duplicates).await?;
        }

        jvm.put_field(&mut target, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", vertex_buffer)
            .await?;
        jvm.put_field(&mut target, "submeshCount", "I", submesh_count).await?;
        jvm.put_field(&mut target, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;", index_array)
            .await?;
        jvm.put_field(&mut target, "appearances", "[Ljavax/microedition/m3g/Appearance;", appearance_array)
            .await?;
        if class_name == "javax/microedition/m3g/SkinnedMesh" {
            let skeleton: ClassInstanceRef<Group> = jvm
                .get_field(source, "skeleton", "Ljavax/microedition/m3g/Group;")
                .await
                .unwrap_or_else(|_| null_ref());
            let skeleton = Self::duplicate_typed_ref(jvm, skeleton, map).await?;
            jvm.put_field(&mut target, "skeleton", "Ljavax/microedition/m3g/Group;", skeleton).await?;
            Self::copy_skinned_bones(jvm, source, &mut target, map).await?;
        }
        if class_name == "javax/microedition/m3g/MorphingMesh" {
            Self::copy_morph_targets(jvm, source, &mut target).await?;
        }
        Ok(duplicate)
    }
    pub(super) async fn duplicate_camera(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Camera> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_camera: ClassInstanceRef<Camera> = cast_ref(source);
        let source_node = cast_ref(&source_camera);
        let mut target_node = cast_ref(&target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
        Self::copy_i32_field(jvm, &source_camera, &mut target, "projectionMode", "I", Camera::PERSPECTIVE).await?;
        Self::copy_f32_field(jvm, &source_camera, &mut target, "fovy", 45.0).await?;
        Self::copy_f32_field(jvm, &source_camera, &mut target, "parallelHeight", 1.0).await?;
        Self::copy_f32_field(jvm, &source_camera, &mut target, "aspect", 1.0).await?;
        Self::copy_f32_field(jvm, &source_camera, &mut target, "near", 1.0).await?;
        Self::copy_f32_field(jvm, &source_camera, &mut target, "far", 1000.0).await?;
        let projection = Camera::generic_projection(jvm, &source_camera)
            .await
            .unwrap_or_else(|_| identity_matrix());
        Camera::put_generic_projection(jvm, &mut target, projection).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_sprite3d(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let source_sprite: ClassInstanceRef<Sprite3D> = cast_ref(source);
        let scaled = jvm.get_field(&source_sprite, "scaled", "Z").await.unwrap_or(false);
        let image: ClassInstanceRef<Image2D> = jvm
            .get_field(&source_sprite, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        let appearance: ClassInstanceRef<Appearance> = jvm
            .get_field(&source_sprite, "appearance", "Ljavax/microedition/m3g/Appearance;")
            .await
            .unwrap_or_else(|_| null_ref());
        let appearance = Self::duplicate_typed_ref(jvm, appearance, map).await?;
        let mut target: ClassInstanceRef<Sprite3D> = jvm
            .new_class(
                &class_name,
                "(ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V",
                (scaled, image.clone(), appearance.clone()),
            )
            .await?
            .into();
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_node = cast_ref(&source_sprite);
        let mut target_node = cast_ref(&target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
        jvm.put_field(&mut target, "scaled", "Z", scaled).await?;
        jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        jvm.put_field(&mut target, "appearance", "Ljavax/microedition/m3g/Appearance;", appearance)
            .await?;
        Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropX", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropY", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropW", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_sprite, &mut target, "cropH", "I", 0).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_light(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Light> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_node = cast_ref(source);
        let mut target_node = cast_ref(&target);
        Self::copy_node_fields(jvm, &source_node, &mut target_node).await?;
        let source_light: ClassInstanceRef<Light> = cast_ref(source);
        Self::copy_f32_field(jvm, &source_light, &mut target, "constantAttenuation", 1.0).await?;
        Self::copy_f32_field(jvm, &source_light, &mut target, "linearAttenuation", 0.0).await?;
        Self::copy_f32_field(jvm, &source_light, &mut target, "quadraticAttenuation", 0.0).await?;
        Self::copy_i32_field(jvm, &source_light, &mut target, "color", "I", 0x00ff_ffff).await?;
        Self::copy_f32_field(jvm, &source_light, &mut target, "intensity", 1.0).await?;
        Self::copy_i32_field(jvm, &source_light, &mut target, "mode", "I", Light::DIRECTIONAL).await?;
        Self::copy_f32_field(jvm, &source_light, &mut target, "spotAngle", 45.0).await?;
        Self::copy_f32_field(jvm, &source_light, &mut target, "spotExponent", 0.0).await?;
        Ok(duplicate)
    }
}
