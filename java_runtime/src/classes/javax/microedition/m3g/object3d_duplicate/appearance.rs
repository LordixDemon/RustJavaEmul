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
    pub(super) async fn duplicate_appearance(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Appearance> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;

        let source_appearance: ClassInstanceRef<Appearance> = cast_ref(source);
        let compositing: ClassInstanceRef<CompositingMode> = jvm
            .get_field(&source_appearance, "compositingMode", "Ljavax/microedition/m3g/CompositingMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        let fog: ClassInstanceRef<Fog> = jvm
            .get_field(&source_appearance, "fog", "Ljavax/microedition/m3g/Fog;")
            .await
            .unwrap_or_else(|_| null_ref());
        let polygon: ClassInstanceRef<PolygonMode> = jvm
            .get_field(&source_appearance, "polygonMode", "Ljavax/microedition/m3g/PolygonMode;")
            .await
            .unwrap_or_else(|_| null_ref());
        let material: ClassInstanceRef<Material> = jvm
            .get_field(&source_appearance, "material", "Ljavax/microedition/m3g/Material;")
            .await
            .unwrap_or_else(|_| null_ref());
        let texture: ClassInstanceRef<Texture2D> = jvm
            .get_field(&source_appearance, "texture0", "Ljavax/microedition/m3g/Texture2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        let texture1: ClassInstanceRef<Texture2D> = jvm
            .get_field(&source_appearance, "texture1", "Ljavax/microedition/m3g/Texture2D;")
            .await
            .unwrap_or_else(|_| null_ref());

        Self::copy_i32_field(jvm, &source_appearance, &mut target, "layer", "I", 0).await?;
        jvm.put_field(
            &mut target,
            "compositingMode",
            "Ljavax/microedition/m3g/CompositingMode;",
            Self::duplicate_typed_ref(jvm, compositing, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "fog",
            "Ljavax/microedition/m3g/Fog;",
            Self::duplicate_typed_ref(jvm, fog, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "polygonMode",
            "Ljavax/microedition/m3g/PolygonMode;",
            Self::duplicate_typed_ref(jvm, polygon, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "material",
            "Ljavax/microedition/m3g/Material;",
            Self::duplicate_typed_ref(jvm, material, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "texture0",
            "Ljavax/microedition/m3g/Texture2D;",
            Self::duplicate_typed_ref(jvm, texture, map).await?,
        )
        .await?;
        jvm.put_field(
            &mut target,
            "texture1",
            "Ljavax/microedition/m3g/Texture2D;",
            Self::duplicate_typed_ref(jvm, texture1, map).await?,
        )
        .await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_background(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Background> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_background: ClassInstanceRef<Background> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_background, &mut target, "color", "I", 0xff000000u32 as i32).await?;
        Self::copy_bool_field(jvm, &source_background, &mut target, "colorClear", true).await?;
        Self::copy_bool_field(jvm, &source_background, &mut target, "depthClear", true).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "imageModeX", "I", 32).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "imageModeY", "I", 32).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "cropX", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "cropY", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "cropW", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_background, &mut target, "cropH", "I", 0).await?;
        let image: ClassInstanceRef<Image2D> = jvm
            .get_field(&source_background, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_compositing_mode(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<CompositingMode> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_mode: ClassInstanceRef<CompositingMode> = cast_ref(source);
        Self::copy_f32_field(jvm, &source_mode, &mut target, "alphaThreshold", 0.0).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "alphaWrite", true).await?;
        Self::copy_i32_field(jvm, &source_mode, &mut target, "blending", "I", CompositingMode::REPLACE).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "colorWrite", true).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "depthTest", true).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "depthWrite", true).await?;
        Self::copy_f32_field(jvm, &source_mode, &mut target, "depthOffsetFactor", 0.0).await?;
        Self::copy_f32_field(jvm, &source_mode, &mut target, "depthOffsetUnits", 0.0).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_fog(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Fog> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_fog: ClassInstanceRef<Fog> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_fog, &mut target, "color", "I", 0).await?;
        Self::copy_f32_field(jvm, &source_fog, &mut target, "density", 1.0).await?;
        Self::copy_i32_field(jvm, &source_fog, &mut target, "mode", "I", Fog::LINEAR).await?;
        Self::copy_f32_field(jvm, &source_fog, &mut target, "near", 0.0).await?;
        Self::copy_f32_field(jvm, &source_fog, &mut target, "far", 0.0).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_polygon_mode(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<PolygonMode> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_mode: ClassInstanceRef<PolygonMode> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_mode, &mut target, "culling", "I", 160).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "localCameraLighting", false).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "perspectiveCorrection", true).await?;
        Self::copy_i32_field(jvm, &source_mode, &mut target, "shading", "I", PolygonMode::SHADE_SMOOTH).await?;
        Self::copy_bool_field(jvm, &source_mode, &mut target, "twoSidedLighting", false).await?;
        Self::copy_i32_field(jvm, &source_mode, &mut target, "winding", "I", 168).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_texture2d(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Texture2D> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        let source_texture: ClassInstanceRef<Texture2D> = cast_ref(source);
        let source_transformable = cast_ref(&source_texture);
        let mut target_transformable = cast_ref(&target);
        let mut duplicate_object = duplicate.clone();
        Self::copy_object3d_fields(jvm, source, &mut duplicate_object).await?;
        Self::copy_transformable_fields(jvm, &source_transformable, &mut target_transformable).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "blendColor", "I", 0).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "blending", "I", Texture2D::FUNC_MODULATE).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "wrappingS", "I", Texture2D::WRAP_REPEAT).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "wrappingT", "I", Texture2D::WRAP_REPEAT).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "levelFilter", "I", Texture2D::FILTER_BASE_LEVEL).await?;
        Self::copy_i32_field(jvm, &source_texture, &mut target, "imageFilter", "I", Texture2D::FILTER_NEAREST).await?;
        let image: ClassInstanceRef<Image2D> = jvm
            .get_field(&source_texture, "image", "Ljavax/microedition/m3g/Image2D;")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "image", "Ljavax/microedition/m3g/Image2D;", image).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_image2d(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Image2D> = Self::new_object(jvm, &class_name).await?;
        let duplicate = cast_ref(&target);
        map.insert(source, &duplicate);
        Self::copy_object3d_fields(jvm, source, &mut cast_ref(&target)).await?;
        let source_image: ClassInstanceRef<Image2D> = cast_ref(source);
        Self::copy_i32_field(jvm, &source_image, &mut target, "format", "I", Image2D::RGBA).await?;
        Self::copy_i32_field(jvm, &source_image, &mut target, "width", "I", 1).await?;
        Self::copy_i32_field(jvm, &source_image, &mut target, "height", "I", 1).await?;
        Self::copy_bool_field(jvm, &source_image, &mut target, "mutable", false).await?;
        let image: ClassInstanceRef<Image> = jvm
            .get_field(&source_image, "image", "Ljavax/microedition/lcdui/Image;")
            .await
            .unwrap_or_else(|_| null_ref());
        jvm.put_field(&mut target, "image", "Ljavax/microedition/lcdui/Image;", image).await?;
        Ok(duplicate)
    }
    pub(super) async fn duplicate_material_or_index_buffer(
        jvm: &Jvm,
        source: &ClassInstanceRef<Object3D>,
        map: &mut M3gDuplicateMap,
        class_name: &str,
    ) -> Result<ClassInstanceRef<Object3D>> {
        let mut target: ClassInstanceRef<Object3D> = jvm.new_class(&class_name, "()V", ()).await?.into();
        map.insert(source, &target);
        Self::copy_object3d_fields(jvm, source, &mut target).await?;
        if class_name == "javax/microedition/m3g/Material" {
            let source_material: ClassInstanceRef<Material> = cast_ref(source);
            let mut target_material: ClassInstanceRef<Material> = cast_ref(&target);
            Self::copy_i32_field(jvm, &source_material, &mut target_material, "ambientColor", "I", 0x0033_3333).await?;
            Self::copy_i32_field(jvm, &source_material, &mut target_material, "diffuseColor", "I", 0xffff_ffffu32 as i32).await?;
            Self::copy_i32_field(jvm, &source_material, &mut target_material, "emissiveColor", "I", 0).await?;
            Self::copy_i32_field(jvm, &source_material, &mut target_material, "specularColor", "I", 0).await?;
            Self::copy_f32_field(jvm, &source_material, &mut target_material, "shininess", 0.0).await?;
            Self::copy_bool_field(jvm, &source_material, &mut target_material, "vertexColorTracking", false).await?;
        }
        Ok(target)
    }
}
