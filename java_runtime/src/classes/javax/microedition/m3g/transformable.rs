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

impl Transformable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Transformable",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getCompositeTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::get_composite_transform,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::get_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("getOrientation", "([F)V", Self::get_orientation, Default::default()),
                JavaMethodProto::new("getScale", "([F)V", Self::get_scale, Default::default()),
                JavaMethodProto::new("getTranslation", "([F)V", Self::get_translation, Default::default()),
                JavaMethodProto::new("postRotate", "(FFFF)V", Self::post_rotate, Default::default()),
                JavaMethodProto::new("preRotate", "(FFFF)V", Self::pre_rotate, Default::default()),
                JavaMethodProto::new("scale", "(FFF)V", Self::scale, Default::default()),
                JavaMethodProto::new("setOrientation", "(FFFF)V", Self::set_orientation, Default::default()),
                JavaMethodProto::new("setScale", "(FFF)V", Self::set_scale, Default::default()),
                JavaMethodProto::new(
                    "setTransform",
                    "(Ljavax/microedition/m3g/Transform;)V",
                    Self::set_transform,
                    Default::default(),
                ),
                JavaMethodProto::new("setTranslation", "(FFF)V", Self::set_translation, Default::default()),
                JavaMethodProto::new("translate", "(FFF)V", Self::translate, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("translationX", "F", Default::default()),
                JavaFieldProto::new("translationY", "F", Default::default()),
                JavaFieldProto::new("translationZ", "F", Default::default()),
                JavaFieldProto::new("scaleX", "F", Default::default()),
                JavaFieldProto::new("scaleY", "F", Default::default()),
                JavaFieldProto::new("scaleZ", "F", Default::default()),
                JavaFieldProto::new("orientationAngle", "F", Default::default()),
                JavaFieldProto::new("orientationX", "F", Default::default()),
                JavaFieldProto::new("orientationY", "F", Default::default()),
                JavaFieldProto::new("orientationZ", "F", Default::default()),
                JavaFieldProto::new("transform", "[F", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        Self::set_translation_fields(jvm, &mut this, 0.0, 0.0, 0.0).await?;
        Self::set_scale_fields(jvm, &mut this, 1.0, 1.0, 1.0).await?;
        Self::set_orientation_fields(jvm, &mut this, 0.0, 0.0, 0.0, 1.0).await?;
        let mut transform = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut transform, &identity_matrix()).await?;
        jvm.put_field(&mut this, "transform", "[F", transform).await
    }

    pub(super) async fn get_composite_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        if this.is_null() || dst.is_null() {
            return Err(jvm
                .exception("java/lang/NullPointerException", "Transformable.getCompositeTransform")
                .await);
        }
        let matrix = Self::local_matrix(jvm, &this).await?;
        Transform::put_matrix(jvm, &mut dst, matrix).await
    }

    pub(super) async fn get_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        if this.is_null() || dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getTransform").await);
        }
        let matrix = Self::generic_matrix(jvm, &this).await?;
        Transform::put_matrix(jvm, &mut dst, matrix).await
    }

    pub(super) async fn get_orientation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getOrientation").await);
        }
        if jvm.array_length(&dst).await? < 4 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "orientation array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "orientationAngle", "F").await?,
            jvm.get_field::<f32>(&this, "orientationX", "F").await?,
            jvm.get_field::<f32>(&this, "orientationY", "F").await?,
            jvm.get_field::<f32>(&this, "orientationZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    pub(super) async fn get_scale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getScale").await);
        }
        if jvm.array_length(&dst).await? < 3 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "scale array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "scaleX", "F").await?,
            jvm.get_field::<f32>(&this, "scaleY", "F").await?,
            jvm.get_field::<f32>(&this, "scaleZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    pub(super) async fn get_translation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut dst: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if dst.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Transformable.getTranslation").await);
        }
        if jvm.array_length(&dst).await? < 3 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "translation array too small").await);
        }
        let values: Vec<f32> = vec![
            jvm.get_field::<f32>(&this, "translationX", "F").await?,
            jvm.get_field::<f32>(&this, "translationY", "F").await?,
            jvm.get_field::<f32>(&this, "translationZ", "F").await?,
        ];
        store_raw_f32_array(jvm, &mut dst, &values).await
    }

    pub(super) async fn post_rotate(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        angle: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) -> Result<()> {
        let matrix = Self::orientation_matrix(jvm, &this).await?;
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(multiply_matrix(matrix, rotation_matrix(angle, ax, ay, az)));
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    pub(super) async fn pre_rotate(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        angle: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) -> Result<()> {
        let matrix = Self::orientation_matrix(jvm, &this).await?;
        let (angle, ax, ay, az) = rotation_matrix_to_axis_angle(multiply_matrix(rotation_matrix(angle, ax, ay, az), matrix));
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    pub(super) async fn scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        let old_x: f32 = jvm.get_field(&this, "scaleX", "F").await?;
        let old_y: f32 = jvm.get_field(&this, "scaleY", "F").await?;
        let old_z: f32 = jvm.get_field(&this, "scaleZ", "F").await?;
        Self::set_scale_fields(jvm, &mut this, old_x * sx, old_y * sy, old_z * sz).await
    }

    pub(super) async fn set_orientation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        angle: f32,
        ax: f32,
        ay: f32,
        az: f32,
    ) -> Result<()> {
        Self::set_orientation_fields(jvm, &mut this, angle, ax, ay, az).await
    }

    pub(super) async fn set_scale(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        Self::set_scale_fields(jvm, &mut this, sx, sy, sz).await
    }

    pub(super) async fn set_transform(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        transform: ClassInstanceRef<Transform>,
    ) -> Result<()> {
        let matrix = Transform::matrix(jvm, &transform).await?;
        Self::put_generic_matrix(jvm, &mut this, matrix).await
    }

    pub(super) async fn set_translation(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        tx: f32,
        ty: f32,
        tz: f32,
    ) -> Result<()> {
        Self::set_translation_fields(jvm, &mut this, tx, ty, tz).await
    }

    pub(super) async fn translate(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        let old_x: f32 = jvm.get_field(&this, "translationX", "F").await?;
        let old_y: f32 = jvm.get_field(&this, "translationY", "F").await?;
        let old_z: f32 = jvm.get_field(&this, "translationZ", "F").await?;
        Self::set_translation_fields(jvm, &mut this, old_x + tx, old_y + ty, old_z + tz).await
    }

    pub(super) async fn set_translation_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, tx: f32, ty: f32, tz: f32) -> Result<()> {
        jvm.put_field(this, "translationX", "F", tx).await?;
        jvm.put_field(this, "translationY", "F", ty).await?;
        jvm.put_field(this, "translationZ", "F", tz).await
    }

    pub(super) async fn set_scale_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, sx: f32, sy: f32, sz: f32) -> Result<()> {
        jvm.put_field(this, "scaleX", "F", sx).await?;
        jvm.put_field(this, "scaleY", "F", sy).await?;
        jvm.put_field(this, "scaleZ", "F", sz).await
    }

    pub(super) async fn set_orientation_fields(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, angle: f32, ax: f32, ay: f32, az: f32) -> Result<()> {
        jvm.put_field(this, "orientationAngle", "F", angle).await?;
        jvm.put_field(this, "orientationX", "F", ax).await?;
        jvm.put_field(this, "orientationY", "F", ay).await?;
        jvm.put_field(this, "orientationZ", "F", az).await
    }

    pub(super) async fn generic_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let matrix: ClassInstanceRef<Array<f32>> = jvm.get_field(this, "transform", "[F").await?;
        let values = raw_f32_array(jvm, &matrix, 16).await?;
        Ok(matrix_to_array(&values))
    }

    pub(super) async fn put_generic_matrix(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, matrix: [f32; 16]) -> Result<()> {
        let mut array = jvm.instantiate_array("F", 16).await?;
        store_raw_f32_array(jvm, &mut array, &matrix).await?;
        jvm.put_field(this, "transform", "[F", array).await
    }

    pub(super) async fn orientation_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let angle: f32 = jvm.get_field(this, "orientationAngle", "F").await?;
        let ax: f32 = jvm.get_field(this, "orientationX", "F").await?;
        let ay: f32 = jvm.get_field(this, "orientationY", "F").await?;
        let az: f32 = jvm.get_field(this, "orientationZ", "F").await?;
        Ok(rotation_matrix(angle, ax, ay, az))
    }

    pub(super) async fn local_matrix(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<[f32; 16]> {
        let tx: f32 = jvm.get_field(this, "translationX", "F").await?;
        let ty: f32 = jvm.get_field(this, "translationY", "F").await?;
        let tz: f32 = jvm.get_field(this, "translationZ", "F").await?;
        let sx: f32 = jvm.get_field(this, "scaleX", "F").await?;
        let sy: f32 = jvm.get_field(this, "scaleY", "F").await?;
        let sz: f32 = jvm.get_field(this, "scaleZ", "F").await?;
        let orientation = Self::orientation_matrix(jvm, this).await?;
        let generic = Self::generic_matrix(jvm, this).await?;

        Ok(multiply_matrix(
            multiply_matrix(multiply_matrix(translation_matrix(tx, ty, tz), orientation), scale_matrix(sx, sy, sz)),
            generic,
        ))
    }
}
