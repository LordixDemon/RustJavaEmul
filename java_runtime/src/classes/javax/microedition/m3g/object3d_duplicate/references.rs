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
    pub(crate) async fn push_find_references(
        jvm: &Jvm,
        current: &ClassInstanceRef<Object3D>,
        stack: &mut Vec<ClassInstanceRef<Object3D>>,
    ) -> Result<()> {
        let tracks: ClassInstanceRef<Array<ClassInstanceRef<AnimationTrack>>> = jvm
            .get_field(current, "animationTracks", "[Ljavax/microedition/m3g/AnimationTrack;")
            .await
            .unwrap_or_else(|_| null_ref());
        if !tracks.is_null() {
            let count = jvm.array_length(&tracks).await?;
            if count > 0 {
                let values: Vec<ClassInstanceRef<AnimationTrack>> = jvm.load_array(&tracks, 0, count).await?;
                stack.extend(values.iter().rev().map(cast_ref::<AnimationTrack, Object3D>));
            }
        }

        let class_name = current.class_definition().name().to_string();
        if Self::is_class(jvm, current, "javax/microedition/m3g/Node") {
            Self::push_object_field::<Node>(jvm, current, "zReference", "Ljavax/microedition/m3g/Node;", stack).await?;
            Self::push_object_field::<Node>(jvm, current, "yReference", "Ljavax/microedition/m3g/Node;", stack).await?;
        }
        if class_name == "javax/microedition/m3g/Group" || class_name == "javax/microedition/m3g/World" {
            let child_count: i32 = jvm.get_field(current, "childCount", "I").await.unwrap_or(0);
            if child_count > 0 {
                let children: ClassInstanceRef<Array<ClassInstanceRef<Node>>> =
                    jvm.get_field(current, "children", "[Ljavax/microedition/m3g/Node;").await?;
                let children: Vec<ClassInstanceRef<Node>> = jvm.load_array(&children, 0, child_count as usize).await?;
                stack.extend(children.iter().rev().map(cast_ref::<Node, Object3D>));
            }
        }

        match class_name.as_str() {
            "javax/microedition/m3g/World" => {
                Self::push_object_field::<Camera>(jvm, current, "activeCamera", "Ljavax/microedition/m3g/Camera;", stack).await?;
                Self::push_object_field::<Background>(jvm, current, "background", "Ljavax/microedition/m3g/Background;", stack).await?;
            }
            "javax/microedition/m3g/Background" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
            }
            "javax/microedition/m3g/Appearance" => {
                Self::push_object_field::<CompositingMode>(jvm, current, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;", stack)
                    .await?;
                Self::push_object_field::<Fog>(jvm, current, "fog", "Ljavax/microedition/m3g/Fog;", stack).await?;
                Self::push_object_field::<PolygonMode>(jvm, current, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;", stack).await?;
                Self::push_object_field::<Material>(jvm, current, "material", "Ljavax/microedition/m3g/Material;", stack).await?;
                Self::push_object_field::<Texture2D>(jvm, current, "texture0", "Ljavax/microedition/m3g/Texture2D;", stack).await?;
                Self::push_object_field::<Texture2D>(jvm, current, "texture1", "Ljavax/microedition/m3g/Texture2D;", stack).await?;
            }
            "javax/microedition/m3g/Texture2D" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
            }
            "javax/microedition/m3g/Sprite3D" => {
                Self::push_object_field::<Image2D>(jvm, current, "image", "Ljavax/microedition/m3g/Image2D;", stack).await?;
                Self::push_object_field::<Appearance>(jvm, current, "appearance", "Ljavax/microedition/m3g/Appearance;", stack).await?;
            }
            "javax/microedition/m3g/Mesh" | "javax/microedition/m3g/SkinnedMesh" | "javax/microedition/m3g/MorphingMesh" => {
                Self::push_object_field::<VertexBuffer>(jvm, current, "vertexBuffer", "Ljavax/microedition/m3g/VertexBuffer;", stack).await?;
                let count: i32 = jvm.get_field(current, "submeshCount", "I").await.unwrap_or(0);
                if count > 0 {
                    let index_buffers: ClassInstanceRef<Array<ClassInstanceRef<IndexBuffer>>> =
                        jvm.get_field(current, "indexBuffers", "[Ljavax/microedition/m3g/IndexBuffer;").await?;
                    if !index_buffers.is_null() {
                        let values: Vec<ClassInstanceRef<IndexBuffer>> = jvm.load_array(&index_buffers, 0, count as usize).await?;
                        stack.extend(values.iter().rev().map(cast_ref::<IndexBuffer, Object3D>));
                    }
                    let appearances: ClassInstanceRef<Array<ClassInstanceRef<Appearance>>> =
                        jvm.get_field(current, "appearances", "[Ljavax/microedition/m3g/Appearance;").await?;
                    if !appearances.is_null() {
                        let values: Vec<ClassInstanceRef<Appearance>> = jvm.load_array(&appearances, 0, count as usize).await?;
                        stack.extend(values.iter().rev().map(cast_ref::<Appearance, Object3D>));
                    }
                }
                if class_name == "javax/microedition/m3g/SkinnedMesh" {
                    Self::push_object_field::<Group>(jvm, current, "skeleton", "Ljavax/microedition/m3g/Group;", stack).await?;
                    Self::push_array_field::<Node>(jvm, current, "boneNodes", "[Ljavax/microedition/m3g/Node;", stack).await?;
                }
                if class_name == "javax/microedition/m3g/MorphingMesh" {
                    Self::push_array_field::<VertexBuffer>(jvm, current, "targets", "[Ljavax/microedition/m3g/VertexBuffer;", stack).await?;
                }
            }
            "javax/microedition/m3g/VertexBuffer" => {
                Self::push_object_field::<VertexArray>(jvm, current, "positions", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "normals", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "colors", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
                Self::push_object_field::<VertexArray>(jvm, current, "texCoords1", "Ljavax/microedition/m3g/VertexArray;", stack).await?;
            }
            "javax/microedition/m3g/AnimationTrack" => {
                Self::push_object_field::<KeyframeSequence>(jvm, current, "sequence", "Ljavax/microedition/m3g/KeyframeSequence;", stack).await?;
                Self::push_object_field::<AnimationController>(jvm, current, "controller", "Ljavax/microedition/m3g/AnimationController;", stack)
                    .await?;
            }
            _ => {}
        }
        Ok(())
    }

    pub(crate) async fn push_array_field<T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        name: &str,
        descriptor: &str,
        stack: &mut Vec<ClassInstanceRef<Object3D>>,
    ) -> Result<()> {
        let array: ClassInstanceRef<Array<ClassInstanceRef<T>>> = jvm.get_field(source, name, descriptor).await.unwrap_or_else(|_| null_ref());
        if array.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&array).await?;
        if count == 0 {
            return Ok(());
        }
        let values: Vec<ClassInstanceRef<T>> = jvm.load_array(&array, 0, count).await?;
        stack.extend(values.iter().rev().map(cast_ref::<T, Object3D>));
        Ok(())
    }

    pub(crate) async fn push_object_field<T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        name: &str,
        descriptor: &str,
        stack: &mut Vec<ClassInstanceRef<Object3D>>,
    ) -> Result<()> {
        let value: ClassInstanceRef<T> = jvm.get_field(source, name, descriptor).await.unwrap_or_else(|_| null_ref());
        if !value.is_null() {
            stack.push(cast_ref(&value));
        }
        Ok(())
    }

    pub(crate) async fn get_user_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "userID", "I").await
    }

    pub(crate) async fn set_user_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, user_id: i32) -> Result<()> {
        jvm.put_field(&mut this, "userID", "I", user_id).await
    }

    pub(crate) async fn set_user_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        user_object: ClassInstanceRef<Object>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "userObject", "Ljava/lang/Object;", user_object).await
    }

    pub(crate) async fn get_user_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "userObject", "Ljava/lang/Object;").await
    }

    pub(crate) async fn get_references(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut references: ClassInstanceRef<Array<ClassInstanceRef<Object3D>>>,
    ) -> Result<i32> {
        if this.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Object3D.getReferences").await);
        }

        let mut collected = Vec::new();
        Self::push_find_references(jvm, &this, &mut collected).await?;
        let mut unique = Vec::new();
        for item in collected {
            if item.is_null() || same_instance(&item, &this) {
                continue;
            }
            if unique.iter().any(|seen| same_instance(seen, &item)) {
                continue;
            }
            unique.push(item);
        }
        let count = unique.len() as i32;
        if references.is_null() {
            return Ok(count);
        }
        if jvm.array_length(&references).await? < unique.len() {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", "Object3D.getReferences array too small")
                .await);
        }
        jvm.store_array(&mut references, 0, unique).await?;
        Ok(count)
    }
}
