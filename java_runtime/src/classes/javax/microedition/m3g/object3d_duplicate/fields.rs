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
    pub(crate) async fn copy_morph_targets(jvm: &Jvm, source: &ClassInstanceRef<Object3D>, target: &mut ClassInstanceRef<Mesh>) -> Result<()> {
        let targets: ClassInstanceRef<Array<ClassInstanceRef<VertexBuffer>>> = jvm
            .get_field(source, "targets", "[Ljavax/microedition/m3g/VertexBuffer;")
            .await
            .unwrap_or_else(|_| null_ref());
        let weights: ClassInstanceRef<Array<f32>> = jvm.get_field(source, "weights", "[F").await.unwrap_or_else(|_| null_ref());
        jvm.put_field(target, "targets", "[Ljavax/microedition/m3g/VertexBuffer;", targets)
            .await?;
        jvm.put_field(target, "weights", "[F", weights).await
    }

    pub(super) async fn copy_skinned_bones(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        target: &mut ClassInstanceRef<Mesh>,
        map: &mut M3gDuplicateMap,
    ) -> Result<()> {
        let bones: ClassInstanceRef<Array<ClassInstanceRef<Node>>> = jvm
            .get_field(source, "boneNodes", "[Ljavax/microedition/m3g/Node;")
            .await
            .unwrap_or_else(|_| null_ref());
        if bones.is_null() {
            return Ok(());
        }
        let count = jvm.array_length(&bones).await?;
        let source_bones: Vec<ClassInstanceRef<Node>> = jvm.load_array(&bones, 0, count).await?;
        let mut duplicated = Vec::with_capacity(source_bones.len());
        for bone in source_bones {
            duplicated.push(Self::duplicate_typed_ref(jvm, bone, map).await?);
        }
        let mut bone_array = jvm.instantiate_array("Ljavax/microedition/m3g/Node;", duplicated.len()).await?;
        jvm.store_array(&mut bone_array, 0, duplicated).await?;
        jvm.put_field(target, "boneNodes", "[Ljavax/microedition/m3g/Node;", bone_array).await?;
        for (name, descriptor) in [("boneWeights", "[I"), ("boneFirst", "[I"), ("boneCount", "[I"), ("boneBind", "[F")] {
            let array: ClassInstanceRef<Object> = jvm.get_field(source, name, descriptor).await.unwrap_or_else(|_| null_ref());
            jvm.put_field(target, name, descriptor, array).await?;
        }
        Ok(())
    }

    pub(crate) async fn copy_i32_field<S, T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<S>,
        target: &mut ClassInstanceRef<T>,
        name: &str,
        descriptor: &str,
        default: i32,
    ) -> Result<()> {
        let value = jvm.get_field(source, name, descriptor).await.unwrap_or(default);
        jvm.put_field(target, name, descriptor, value).await
    }

    pub(crate) async fn copy_f32_field<S, T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<S>,
        target: &mut ClassInstanceRef<T>,
        name: &str,
        default: f32,
    ) -> Result<()> {
        let value = jvm.get_field(source, name, "F").await.unwrap_or(default);
        jvm.put_field(target, name, "F", value).await
    }

    pub(crate) async fn copy_bool_field<S, T>(
        jvm: &Jvm,
        source: &ClassInstanceRef<S>,
        target: &mut ClassInstanceRef<T>,
        name: &str,
        default: bool,
    ) -> Result<()> {
        let value = jvm.get_field(source, name, "Z").await.unwrap_or(default);
        jvm.put_field(target, name, "Z", value).await
    }
}
