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
    pub(super) async fn duplicate_triangle_strip_array(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<TriangleStripArray> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_array: ClassInstanceRef<TriangleStripArray> = cast_ref(source);
        let indices: ClassInstanceRef<Array<i32>> = jvm.get_field(&source_array, "indices", "[I").await.unwrap_or_else(|_| null_ref());
        let lengths: ClassInstanceRef<Array<i32>> = jvm.get_field(&source_array, "stripLengths", "[I").await.unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "indices", "[I", indices).await?;
        jvm.put_field(&mut target, "stripLengths", "[I", lengths).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_vertex_array(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<VertexArray> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_array: ClassInstanceRef<VertexArray> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_array, &mut target, "componentSize", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_array, &mut target, "componentCount", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_array, &mut target, "vertexCount", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_array, &mut target, "version", "I", 0).await?;
        let byte_data: ClassInstanceRef<Array<i8>> = jvm.get_field(&source_array, "byteData", "[B").await.unwrap_or_else(|_| null_ref());
        let short_data: ClassInstanceRef<Array<i16>> = jvm.get_field(&source_array, "shortData", "[S").await.unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "byteData", "[B", byte_data).await?;
        jvm.put_field(&mut target, "shortData", "[S", short_data).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_vertex_buffer(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<VertexBuffer> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_buffer: ClassInstanceRef<VertexBuffer> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_buffer, &mut target, "defaultColor", "I", 0x00ff_ffff).await?;
        Self::copy_f32_field(jvm, &source_buffer, &mut target, "positionScale", 1.0).await?;
        Self::copy_f32_field(jvm, &source_buffer, &mut target, "texScale", 1.0).await?;
        Self::copy_f32_field(jvm, &source_buffer, &mut target, "texScale1", 1.0).await?;
        let positions: ClassInstanceRef<VertexArray> = jvm
            .get_field(&source_buffer, "positions", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let normals: ClassInstanceRef<VertexArray> = jvm
            .get_field(&source_buffer, "normals", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let colors: ClassInstanceRef<VertexArray> = jvm
            .get_field(&source_buffer, "colors", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let tex_coords: ClassInstanceRef<VertexArray> = jvm
            .get_field(&source_buffer, "texCoords0", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let tex_coords1: ClassInstanceRef<VertexArray> = jvm
            .get_field(&source_buffer, "texCoords1", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let position_bias: ClassInstanceRef<Array<f32>> = jvm.get_field(&source_buffer, "positionBias", "[F").await.unwrap_or_else(|_| null_ref());
        let tex_bias: ClassInstanceRef<Array<f32>> = jvm.get_field(&source_buffer, "texBias", "[F").await.unwrap_or_else(|_| null_ref());
        let tex_bias1: ClassInstanceRef<Array<f32>> = jvm.get_field(&source_buffer, "texBias1", "[F").await.unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
            .await?;
        jvm.put_field(&mut target, "normals", "Ljavax/microedition/m3g/VertexArray;", normals)
            .await?;
        jvm.put_field(&mut target, "colors", "Ljavax/microedition/m3g/VertexArray;", colors)
            .await?;
        jvm.put_field(&mut target, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", tex_coords)
            .await?;
        jvm.put_field(&mut target, "texCoords1", "Ljavax/microedition/m3g/VertexArray;", tex_coords1)
            .await?;
        jvm.put_field(&mut target, "positionBias", "[F", position_bias).await?;
        jvm.put_field(&mut target, "texBias", "[F", tex_bias).await?;
        jvm.put_field(&mut target, "texBias1", "[F", tex_bias1).await?;
        Ok(duplicate)
    }
}
