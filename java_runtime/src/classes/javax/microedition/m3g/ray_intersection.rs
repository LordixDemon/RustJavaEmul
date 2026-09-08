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

impl RayIntersection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/RayIntersection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getDistance", "()F", Self::get_distance, Default::default()),
                JavaMethodProto::new(
                    "getIntersected",
                    "()Ljavax/microedition/m3g/Node;",
                    Self::get_intersected,
                    Default::default(),
                ),
                JavaMethodProto::new("getNormalX", "()F", Self::get_normal_x, Default::default()),
                JavaMethodProto::new("getNormalY", "()F", Self::get_normal_y, Default::default()),
                JavaMethodProto::new("getNormalZ", "()F", Self::get_normal_z, Default::default()),
                JavaMethodProto::new("getRay", "([F)V", Self::get_ray, Default::default()),
                JavaMethodProto::new("getSubmeshIndex", "()I", Self::get_submesh_index, Default::default()),
                JavaMethodProto::new("getTextureS", "(I)F", Self::get_texture_s, Default::default()),
                JavaMethodProto::new("getTextureT", "(I)F", Self::get_texture_t, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("intersected", "Ljavax/microedition/m3g/Node;", Default::default()),
                JavaFieldProto::new("distance", "F", Default::default()),
                JavaFieldProto::new("submeshIndex", "I", Default::default()),
                JavaFieldProto::new("textureS", "[F", Default::default()),
                JavaFieldProto::new("textureT", "[F", Default::default()),
                JavaFieldProto::new("normal", "[F", Default::default()),
                JavaFieldProto::new("ray", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "intersected", "Ljavax/microedition/m3g/Node;", null_ref::<Node>())
            .await?;
        jvm.put_field(&mut this, "distance", "F", 0.0f32).await?;
        jvm.put_field(&mut this, "submeshIndex", "I", 0).await?;
        let texture_s = jvm.instantiate_array("F", 2).await?;
        let texture_t = jvm.instantiate_array("F", 2).await?;
        let mut normal = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut normal, 0, vec![0.0f32, 0.0, 1.0]).await?;
        let mut ray = jvm.instantiate_array("F", 6).await?;
        jvm.store_array(&mut ray, 0, vec![0.0f32, 0.0, 0.0, 0.0, 0.0, 1.0]).await?;
        jvm.put_field(&mut this, "textureS", "[F", texture_s).await?;
        jvm.put_field(&mut this, "textureT", "[F", texture_t).await?;
        jvm.put_field(&mut this, "normal", "[F", normal).await?;
        jvm.put_field(&mut this, "ray", "[F", ray).await
    }

    pub(super) async fn get_distance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "distance", "F").await
    }

    pub(super) async fn get_intersected(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Node>> {
        jvm.get_field(&this, "intersected", "Ljavax/microedition/m3g/Node;").await
    }

    pub(super) async fn get_normal_x(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 0).await
    }

    pub(super) async fn get_normal_y(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 1).await
    }

    pub(super) async fn get_normal_z(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Self::array_value(jvm, &this, "normal", 2).await
    }

    pub(super) async fn get_ray(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut out: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "RayIntersection.getRay").await);
        }
        if jvm.array_length(&out).await? < 6 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "ray array too small").await);
        }
        let ray: ClassInstanceRef<Array<f32>> = jvm.get_field(&this, "ray", "[F").await?;
        let values: Vec<f32> = jvm.load_array(&ray, 0, 6).await?;
        jvm.store_array(&mut out, 0, values).await
    }

    pub(super) async fn get_submesh_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "submeshIndex", "I").await
    }

    pub(super) async fn get_texture_s(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<f32> {
        Self::texture_value(jvm, &this, "textureS", index).await
    }

    pub(super) async fn get_texture_t(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<f32> {
        Self::texture_value(jvm, &this, "textureT", index).await
    }

    pub(super) async fn fill(
        jvm: &Jvm,
        this: &mut ClassInstanceRef<Self>,
        intersected: ClassInstanceRef<Node>,
        hit: &M3gPickHit,
        ray: [f32; 6],
    ) -> Result<()> {
        jvm.put_field(this, "intersected", "Ljavax/microedition/m3g/Node;", intersected).await?;
        jvm.put_field(this, "distance", "F", hit.distance).await?;
        jvm.put_field(this, "submeshIndex", "I", hit.submesh_index).await?;
        let mut texture_s = jvm.instantiate_array("F", 2).await?;
        jvm.store_array(&mut texture_s, 0, vec![hit.texture_s, hit.texture_s1]).await?;
        let mut texture_t = jvm.instantiate_array("F", 2).await?;
        jvm.store_array(&mut texture_t, 0, vec![hit.texture_t, hit.texture_t1]).await?;
        let mut normal = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut normal, 0, hit.normal.to_vec()).await?;
        let mut ray_array = jvm.instantiate_array("F", 6).await?;
        jvm.store_array(&mut ray_array, 0, ray).await?;
        jvm.put_field(this, "textureS", "[F", texture_s).await?;
        jvm.put_field(this, "textureT", "[F", texture_t).await?;
        jvm.put_field(this, "normal", "[F", normal).await?;
        jvm.put_field(this, "ray", "[F", ray_array).await
    }

    pub(super) async fn texture_value(jvm: &Jvm, this: &ClassInstanceRef<Self>, field: &str, index: i32) -> Result<f32> {
        if !(0..2).contains(&index) {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        Self::array_value(jvm, this, field, index as usize).await
    }

    pub(super) async fn array_value(jvm: &Jvm, this: &ClassInstanceRef<Self>, field: &str, index: usize) -> Result<f32> {
        let array: ClassInstanceRef<Array<f32>> = jvm.get_field(this, field, "[F").await?;
        Ok(jvm.load_array(&array, index, 1).await?.into_iter().next().unwrap_or(0.0))
    }
}
