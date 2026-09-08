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

impl VertexBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/VertexBuffer",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getColors",
                    "()Ljavax/microedition/m3g/VertexArray;",
                    Self::get_colors,
                    Default::default(),
                ),
                JavaMethodProto::new("getDefaultColor", "()I", Self::get_default_color, Default::default()),
                JavaMethodProto::new(
                    "getNormals",
                    "()Ljavax/microedition/m3g/VertexArray;",
                    Self::get_normals,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getPositions",
                    "([F)Ljavax/microedition/m3g/VertexArray;",
                    Self::get_positions,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getTexCoords",
                    "(I[F)Ljavax/microedition/m3g/VertexArray;",
                    Self::get_tex_coords,
                    Default::default(),
                ),
                JavaMethodProto::new("getVertexCount", "()I", Self::get_vertex_count, Default::default()),
                JavaMethodProto::new(
                    "setColors",
                    "(Ljavax/microedition/m3g/VertexArray;)V",
                    Self::set_colors,
                    Default::default(),
                ),
                JavaMethodProto::new("setDefaultColor", "(I)V", Self::set_default_color, Default::default()),
                JavaMethodProto::new(
                    "setNormals",
                    "(Ljavax/microedition/m3g/VertexArray;)V",
                    Self::set_normals,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setPositions",
                    "(Ljavax/microedition/m3g/VertexArray;F[F)V",
                    Self::set_positions,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setTexCoords",
                    "(ILjavax/microedition/m3g/VertexArray;F[F)V",
                    Self::set_tex_coords,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("defaultColor", "I", Default::default()),
                JavaFieldProto::new("positions", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("normals", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("colors", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("texCoords0", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("texCoords1", "Ljavax/microedition/m3g/VertexArray;", Default::default()),
                JavaFieldProto::new("positionScale", "F", Default::default()),
                JavaFieldProto::new("positionBias", "[F", Default::default()),
                JavaFieldProto::new("texScale", "F", Default::default()),
                JavaFieldProto::new("texBias", "[F", Default::default()),
                JavaFieldProto::new("texScale1", "F", Default::default()),
                JavaFieldProto::new("texBias1", "[F", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "defaultColor", "I", 0xffff_ffffu32 as i32).await?;
        jvm.put_field(&mut this, "positions", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "normals", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "colors", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "texCoords0", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "texCoords1", "Ljavax/microedition/m3g/VertexArray;", null_ref::<VertexArray>())
            .await?;
        jvm.put_field(&mut this, "positionScale", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "texScale", "F", 1.0f32).await?;
        jvm.put_field(&mut this, "texScale1", "F", 1.0f32).await?;
        let mut position_bias = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut position_bias, 0, vec![0.0f32, 0.0, 0.0]).await?;
        let mut tex_bias = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut tex_bias, 0, vec![0.0f32, 0.0, 0.0]).await?;
        jvm.put_field(&mut this, "positionBias", "[F", position_bias).await?;
        jvm.put_field(&mut this, "texBias", "[F", tex_bias).await?;
        let mut tex_bias1 = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut tex_bias1, 0, vec![0.0f32, 0.0, 0.0]).await?;
        jvm.put_field(&mut this, "texBias1", "[F", tex_bias1).await
    }

    pub(super) async fn get_colors(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexArray>> {
        jvm.get_field(&this, "colors", "Ljavax/microedition/m3g/VertexArray;").await
    }

    pub(super) async fn get_normals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<VertexArray>> {
        jvm.get_field(&this, "normals", "Ljavax/microedition/m3g/VertexArray;").await
    }

    pub(super) async fn get_positions(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        scale_bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<ClassInstanceRef<VertexArray>> {
        if !scale_bias.is_null() {
            Self::write_scale_bias(jvm, scale_bias, &this, "positionScale", "positionBias", 4).await?;
        }
        jvm.get_field(&this, "positions", "Ljavax/microedition/m3g/VertexArray;").await
    }

    pub(super) async fn get_tex_coords(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        unit: i32,
        scale_bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<ClassInstanceRef<VertexArray>> {
        if unit < 0 || unit >= M3G_TEXTURE_UNITS {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        let (coords_field, scale_field, bias_field) = if unit == 0 {
            ("texCoords0", "texScale", "texBias")
        } else {
            ("texCoords1", "texScale1", "texBias1")
        };
        let tex_coords: ClassInstanceRef<VertexArray> = jvm.get_field(&this, coords_field, "Ljavax/microedition/m3g/VertexArray;").await?;
        if !scale_bias.is_null() && !tex_coords.is_null() {
            let component_count: i32 = jvm.get_field(&tex_coords, "componentCount", "I").await.unwrap_or(2);
            Self::write_scale_bias(jvm, scale_bias, &this, scale_field, bias_field, (component_count + 1).max(0) as usize).await?;
        }
        Ok(tex_coords)
    }

    pub(super) async fn get_vertex_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let positions: ClassInstanceRef<VertexArray> = jvm.get_field(&this, "positions", "Ljavax/microedition/m3g/VertexArray;").await?;
        if positions.is_null() {
            Ok(0)
        } else {
            jvm.get_field(&positions, "vertexCount", "I").await
        }
    }

    pub(super) async fn get_default_color(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "defaultColor", "I").await
    }

    pub(super) async fn set_default_color(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, color: i32) -> Result<()> {
        jvm.put_field(&mut this, "defaultColor", "I", color).await
    }

    pub(super) async fn set_positions(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        positions: ClassInstanceRef<VertexArray>,
        scale: f32,
        bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if !positions.is_null() {
            let component_count: i32 = jvm.get_field(&positions, "componentCount", "I").await?;
            if component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "position array must have 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &positions).await?;
        }
        let bias_values = Self::read_bias(jvm, &bias, 3).await?;
        let mut bias_array = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut bias_array, 0, bias_values).await?;
        jvm.put_field(&mut this, "positions", "Ljavax/microedition/m3g/VertexArray;", positions)
            .await?;
        jvm.put_field(&mut this, "positionScale", "F", scale).await?;
        jvm.put_field(&mut this, "positionBias", "[F", bias_array).await
    }

    pub(super) async fn set_tex_coords(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        unit: i32,
        tex_coords: ClassInstanceRef<VertexArray>,
        scale: f32,
        bias: ClassInstanceRef<Array<f32>>,
    ) -> Result<()> {
        if unit < 0 || unit >= M3G_TEXTURE_UNITS {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "texture unit").await);
        }
        let (coords_field, scale_field, bias_field) = if unit == 0 {
            ("texCoords0", "texScale", "texBias")
        } else {
            ("texCoords1", "texScale1", "texBias1")
        };
        let bias_len = if tex_coords.is_null() {
            3
        } else {
            let component_count: i32 = jvm.get_field(&tex_coords, "componentCount", "I").await?;
            if component_count != 2 && component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "texcoord array must have 2 or 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &tex_coords).await?;
            component_count as usize
        };
        let mut bias_values = Self::read_bias(jvm, &bias, bias_len).await?;
        bias_values.resize(3, 0.0);
        let mut bias_array = jvm.instantiate_array("F", 3).await?;
        jvm.store_array(&mut bias_array, 0, bias_values).await?;
        jvm.put_field(&mut this, coords_field, "Ljavax/microedition/m3g/VertexArray;", tex_coords)
            .await?;
        jvm.put_field(&mut this, scale_field, "F", scale).await?;
        jvm.put_field(&mut this, bias_field, "[F", bias_array).await
    }

    pub(super) async fn set_normals(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        normals: ClassInstanceRef<VertexArray>,
    ) -> Result<()> {
        if !normals.is_null() {
            let component_count: i32 = jvm.get_field(&normals, "componentCount", "I").await?;
            if component_count != 3 {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "normal array must have 3 components")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &normals).await?;
        }
        jvm.put_field(&mut this, "normals", "Ljavax/microedition/m3g/VertexArray;", normals).await
    }

    pub(super) async fn set_colors(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        colors: ClassInstanceRef<VertexArray>,
    ) -> Result<()> {
        if !colors.is_null() {
            let component_size: i32 = jvm.get_field(&colors, "componentSize", "I").await?;
            let component_count: i32 = jvm.get_field(&colors, "componentCount", "I").await?;
            if component_size != 1 || !(3..=4).contains(&component_count) {
                return Err(jvm
                    .exception("java/lang/IllegalArgumentException", "color array must be byte RGB/RGBA")
                    .await);
            }
            Self::check_compatible_vertex_count(jvm, &this, &colors).await?;
        }
        jvm.put_field(&mut this, "colors", "Ljavax/microedition/m3g/VertexArray;", colors).await
    }

    pub(super) async fn check_compatible_vertex_count(jvm: &Jvm, this: &ClassInstanceRef<Self>, array: &ClassInstanceRef<VertexArray>) -> Result<()> {
        let positions: ClassInstanceRef<VertexArray> = jvm
            .get_field(this, "positions", "Ljavax/microedition/m3g/VertexArray;")
            .await
            .unwrap_or_else(|_| null_ref());
        let existing = if positions.is_null() {
            0
        } else {
            jvm.get_field(&positions, "vertexCount", "I").await.unwrap_or(0)
        };
        let incoming: i32 = jvm.get_field(array, "vertexCount", "I").await.unwrap_or(0);
        if existing > 0 && incoming > 0 && existing != incoming {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "vertex count mismatch").await);
        }
        Ok(())
    }

    pub(super) async fn read_bias(jvm: &Jvm, bias: &ClassInstanceRef<Array<f32>>, min_len: usize) -> Result<Vec<f32>> {
        if bias.is_null() {
            return Ok(vec![0.0; min_len]);
        }
        if jvm.array_length(bias).await? < min_len {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "bias array too small").await);
        }
        jvm.load_array(bias, 0, min_len).await
    }

    pub(super) async fn write_scale_bias(
        jvm: &Jvm,
        mut dst: ClassInstanceRef<Array<f32>>,
        this: &ClassInstanceRef<Self>,
        scale_field: &str,
        bias_field: &str,
        required_len: usize,
    ) -> Result<()> {
        if jvm.array_length(&dst).await? < required_len {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "scale bias array too small").await);
        }
        let scale = jvm.get_field::<f32>(this, scale_field, "F").await.unwrap_or(1.0);
        let bias: ClassInstanceRef<Array<f32>> = jvm.get_field(this, bias_field, "[F").await.unwrap_or_else(|_| null_ref());
        let mut values = vec![scale];
        if bias.is_null() {
            values.resize(required_len, 0.0);
        } else {
            let mut bias_values: Vec<f32> = jvm
                .load_array(&bias, 0, jvm.array_length(&bias).await?.min(required_len.saturating_sub(1)))
                .await?;
            values.append(&mut bias_values);
            values.resize(required_len, 0.0);
        }
        jvm.store_array(&mut dst, 0, values).await
    }
}
