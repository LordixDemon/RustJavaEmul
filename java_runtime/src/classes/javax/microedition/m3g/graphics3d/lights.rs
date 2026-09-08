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
    pub(crate) async fn add_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        light: ClassInstanceRef<Light>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<i32> {
        if light.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Graphics3D.addLight").await);
        }
        let mut lights = Self::light_list(jvm, &this).await?;
        let mut transforms = Self::light_transform_list(jvm, &this).await?;
        let index = lights.len() as i32;
        lights.push(light);
        transforms.push(Self::transform_from_optional(jvm, transform).await?);
        Self::store_lights(jvm, &mut this, lights, transforms).await?;
        Ok(index)
    }
    pub(crate) async fn get_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        mut transform: ClassInstanceRef<Transform>,
    ) -> Result<ClassInstanceRef<Light>> {
        let lights = Self::light_list(jvm, &this).await?;
        if index < 0 || index as usize >= lights.len() {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "light index").await);
        }
        if !transform.is_null() {
            let transforms = Self::light_transform_list(jvm, &this).await?;
            if let Some(stored) = transforms.get(index as usize) {
                let matrix = Transform::matrix(jvm, stored).await.unwrap_or_else(|_| identity_matrix());
                Transform::put_matrix(jvm, &mut transform, matrix).await?;
            }
        }
        Ok(lights[index as usize].clone())
    }
    pub(crate) async fn get_light_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(Self::light_list(jvm, &this).await?.len() as i32)
    }
    pub(crate) async fn reset_lights(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let lights = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", 0).await?;
        let transforms = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", 0).await?;
        jvm.put_field(&mut this, "lights", "[Ljavax/microedition/m3g/Light;", lights).await?;
        jvm.put_field(&mut this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", transforms)
            .await
    }
    pub(crate) async fn set_light(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        light: ClassInstanceRef<Light>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        let mut lights = Self::light_list(jvm, &this).await?;
        if index < 0 || index as usize >= lights.len() {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "light index").await);
        }
        let mut transforms = Self::light_transform_list(jvm, &this).await?;
        transforms.resize_with(lights.len(), null_ref::<Transform>);
        lights[index as usize] = light;
        transforms[index as usize] = Self::transform_from_optional(jvm, transform).await?;
        Self::store_lights(jvm, &mut this, lights, transforms).await
    }
    pub(crate) async fn light_list(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<ClassInstanceRef<Light>>> {
        let lights: ClassInstanceRef<Array<ClassInstanceRef<Light>>> = jvm
            .get_field(this, "lights", "[Ljavax/microedition/m3g/Light;")
            .await
            .unwrap_or_else(|_| null_ref());
        if lights.is_null() {
            return Ok(Vec::new());
        }
        jvm.load_array(&lights, 0, jvm.array_length(&lights).await?).await
    }
    pub(crate) async fn light_transform_list(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<ClassInstanceRef<Transform>>> {
        let transforms: ClassInstanceRef<Array<ClassInstanceRef<Transform>>> = jvm
            .get_field(this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;")
            .await
            .unwrap_or_else(|_| null_ref());
        if transforms.is_null() {
            return Ok(Vec::new());
        }
        jvm.load_array(&transforms, 0, jvm.array_length(&transforms).await?).await
    }
    pub(crate) async fn store_lights(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        lights: Vec<ClassInstanceRef<Light>>,
        mut transforms: Vec<ClassInstanceRef<Transform>>,
    ) -> Result<()> {
        transforms.resize_with(lights.len(), null_ref::<Transform>);
        let mut light_array = jvm.instantiate_array("Ljavax/microedition/m3g/Light;", lights.len()).await?;
        jvm.store_array(&mut light_array, 0, lights).await?;
        let mut transform_array = jvm.instantiate_array("Ljavax/microedition/m3g/Transform;", transforms.len()).await?;
        jvm.store_array(&mut transform_array, 0, transforms).await?;
        jvm.put_field(this, "lights", "[Ljavax/microedition/m3g/Light;", light_array).await?;
        jvm.put_field(this, "lightTransforms", "[Ljavax/microedition/m3g/Transform;", transform_array)
            .await
    }
}
