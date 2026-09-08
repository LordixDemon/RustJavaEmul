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

impl super::super::Graphics3D {
    pub(crate) async fn get_camera(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<ClassInstanceRef<Camera>> {
        if !transform.is_null() {
            let matrix = Self::camera_transform(jvm, &this).await.unwrap_or_else(|_| identity_matrix());
            Transform::put_matrix(jvm, &mut transform, matrix).await?;
        }
        jvm.get_field(&this, "camera", "Ljavax/microedition/m3g/Camera;").await
    }
    pub(crate) async fn get_depth_range_far(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthRangeFar", "F").await
    }
    pub(crate) async fn get_depth_range_near(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "depthRangeNear", "F").await
    }
    pub(crate) async fn get_hints(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "hints", "I").await
    }
    pub(crate) async fn get_properties(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Hashtable>> {
        let table: ClassInstanceRef<Hashtable> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
        Self::put_property_i32(jvm, &table, "maxLights", M3G_MAX_LIGHTS).await?;
        Self::put_property_i32(jvm, &table, "maxViewportWidth", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxViewportHeight", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxViewportDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxTextureDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "maxSpriteCropDimension", 4096).await?;
        Self::put_property_i32(jvm, &table, "numTextureUnits", M3G_TEXTURE_UNITS).await?;
        Self::put_property_i32(jvm, &table, "maxTransformsPerVertex", M3G_MAX_TRANSFORMS_PER_VERTEX).await?;
        Self::put_property_bool(jvm, &table, "supportAntialiasing", false).await?;
        Self::put_property_bool(jvm, &table, "supportTrueColor", true).await?;
        Self::put_property_bool(jvm, &table, "supportDithering", false).await?;
        Self::put_property_bool(jvm, &table, "supportMipmapping", false).await?;
        Self::put_property_bool(jvm, &table, "supportPerspectiveCorrection", true).await?;
        Self::put_property_bool(jvm, &table, "supportLocalCameraLighting", true).await?;
        Self::put_property_bool(jvm, &table, "antialiasing", false).await?;
        Self::put_property_bool(jvm, &table, "dithering", false).await?;
        Self::put_property_bool(jvm, &table, "trueColor", true).await?;
        Self::put_property_bool(jvm, &table, "mipmapping", false).await?;
        Self::put_property_string(jvm, &table, "m3gRelease", "rustjava-0.1.0").await?;
        Ok(table)
    }
    pub(crate) async fn set_camera(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        camera: ClassInstanceRef<Camera>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "camera", "Ljavax/microedition/m3g/Camera;", camera).await?;
        if transform.is_null() {
            Self::put_camera_transform(jvm, &mut this, identity_matrix()).await?;
            jvm.put_field(&mut this, "cameraTransformSet", "Z", false).await
        } else {
            let matrix = Transform::matrix(jvm, &transform).await?;
            tracing::debug!(
                target: "rustjava_m3g",
                "m3g.Graphics3D.setCamera matrix tx={:.3} ty={:.3} tz={:.3} xAxis=({:.3},{:.3},{:.3}) yAxis=({:.3},{:.3},{:.3}) zAxis=({:.3},{:.3},{:.3})",
                matrix[3],
                matrix[7],
                matrix[11],
                matrix[0],
                matrix[4],
                matrix[8],
                matrix[1],
                matrix[5],
                matrix[9],
                matrix[2],
                matrix[6],
                matrix[10]
            );
            Self::put_camera_transform(jvm, &mut this, matrix).await?;
            jvm.put_field(&mut this, "cameraTransformSet", "Z", true).await
        }
    }
    pub(crate) async fn set_depth_range(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, near: f32, far: f32) -> Result<()> {
        if !(0.0..=1.0).contains(&near) || !(0.0..=1.0).contains(&far) || near > far {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid depth range").await);
        }
        jvm.put_field(&mut this, "depthRangeNear", "F", near).await?;
        jvm.put_field(&mut this, "depthRangeFar", "F", far).await
    }
    pub(crate) async fn transform_from_optional(jvm: &Jvm, transform: ClassInstanceRef<Transform>) -> Result<ClassInstanceRef<Transform>> {
        let matrix = if transform.is_null() {
            identity_matrix()
        } else {
            Transform::matrix(jvm, &transform).await?
        };
        let mut stored: ClassInstanceRef<Transform> = jvm.new_class("javax/microedition/m3g/Transform", "()V", ()).await?.into();
        Transform::put_matrix(jvm, &mut stored, matrix).await?;
        Ok(stored)
    }
    pub(crate) async fn put_property_bool(jvm: &Jvm, table: &ClassInstanceRef<Hashtable>, name: &str, value: bool) -> Result<()> {
        let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, name).await?.into();
        let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Boolean", "(Z)V", (value,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }
    pub(crate) async fn put_property_i32(jvm: &Jvm, table: &ClassInstanceRef<Hashtable>, name: &str, value: i32) -> Result<()> {
        let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, name).await?.into();
        let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }
    pub(crate) async fn put_property_string(jvm: &Jvm, table: &ClassInstanceRef<Hashtable>, name: &str, value: &str) -> Result<()> {
        let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, name).await?.into();
        let value: ClassInstanceRef<Object> = JavaLangString::from_rust_string(jvm, value).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }
    pub(crate) async fn camera_transform(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "cameraTransform", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }
    pub(crate) async fn put_camera_transform(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        jvm.store_array(&mut array, 0, matrix).await?;
        jvm.put_field(this, "cameraTransform", "[F", array).await
    }
    pub(crate) async fn clear_framebuffers(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(this, "colorBuffer", "[I", null_ref::<Array<i32>>()).await?;
        jvm.put_field(this, "colorBufferValid", "Z", false).await?;
        jvm.put_field(this, "depthBuffer", "[F", null_ref::<Array<f32>>()).await
    }
    pub(crate) async fn load_color_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixel_count: usize) -> Result<(Vec<i32>, bool)> {
        let valid = jvm.get_field::<bool>(this, "colorBufferValid", "Z").await.unwrap_or(false);
        if valid {
            let array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "colorBuffer", "[I").await.unwrap_or_else(|_| null_ref());
            if !array.is_null() && jvm.array_length(&array).await.unwrap_or(0) == pixel_count {
                return Ok((raw_i32_array(jvm, &array, pixel_count).await?, true));
            }
        }
        Ok((vec![0; pixel_count], false))
    }
    pub(crate) async fn store_color_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixels: &[i32]) -> Result<()> {
        let mut array: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "colorBuffer", "[I").await.unwrap_or_else(|_| null_ref());
        if array.is_null() || jvm.array_length(&array).await.unwrap_or(0) != pixels.len() {
            array = jvm.instantiate_array("I", pixels.len()).await?.into();
            let mut this = this.clone();
            jvm.put_field(&mut this, "colorBuffer", "[I", array.clone()).await?;
        }
        store_raw_i32_array(jvm, &mut array, pixels).await?;
        let mut this = this.clone();
        jvm.put_field(&mut this, "colorBufferValid", "Z", true).await
    }
    pub(crate) async fn load_depth_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, pixel_count: usize, clear: bool) -> Result<Vec<f32>> {
        if !clear {
            let array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "depthBuffer", "[F").await.unwrap_or_else(|_| null_ref());
            if !array.is_null() && jvm.array_length(&array).await.unwrap_or(0) == pixel_count {
                return raw_f32_array(jvm, &array, pixel_count).await;
            }
        }
        Ok(vec![f32::INFINITY; pixel_count])
    }
    pub(crate) async fn store_depth_buffer(jvm: &Jvm, this: &ClassInstanceRef<Self>, depth: &[f32]) -> Result<()> {
        let mut array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "depthBuffer", "[F").await.unwrap_or_else(|_| null_ref());
        if array.is_null() || jvm.array_length(&array).await.unwrap_or(0) != depth.len() {
            array = jvm.instantiate_array("F", depth.len()).await?.into();
            let mut this = this.clone();
            jvm.put_field(&mut this, "depthBuffer", "[F", array.clone()).await?;
        }
        store_raw_f32_array(jvm, &mut array, depth).await?;
        Ok(())
    }
}
