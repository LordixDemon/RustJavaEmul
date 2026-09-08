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

impl VertexArray {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/VertexArray",
            parent_class: Some("javax/microedition/m3g/Object3D"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(III)V", Self::init_sized, Default::default()),
                JavaMethodProto::new("get", "(II[B)V", Self::get_bytes, Default::default()),
                JavaMethodProto::new("get", "(II[S)V", Self::get_shorts, Default::default()),
                JavaMethodProto::new("getComponentCount", "()I", Self::get_component_count, Default::default()),
                JavaMethodProto::new("getComponentType", "()I", Self::get_component_type, Default::default()),
                JavaMethodProto::new("getVertexCount", "()I", Self::get_vertex_count, Default::default()),
                JavaMethodProto::new("set", "(II[B)V", Self::set_bytes, Default::default()),
                JavaMethodProto::new("set", "(II[S)V", Self::set_shorts, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("componentSize", "I", Default::default()),
                JavaFieldProto::new("componentCount", "I", Default::default()),
                JavaFieldProto::new("vertexCount", "I", Default::default()),
                JavaFieldProto::new("version", "I", Default::default()),
                JavaFieldProto::new("byteData", "[B", Default::default()),
                JavaFieldProto::new("shortData", "[S", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "javax/microedition/m3g/Object3D", "<init>", "()V", ()).await
    }

    pub(super) async fn init_sized(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        vertex_count: i32,
        component_count: i32,
        component_size: i32,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        if !(1..=65535).contains(&vertex_count) || !(2..=4).contains(&component_count) || (component_size != 1 && component_size != 2) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid vertex array layout").await);
        }
        jvm.put_field(&mut this, "componentSize", "I", component_size).await?;
        jvm.put_field(&mut this, "componentCount", "I", component_count).await?;
        jvm.put_field(&mut this, "vertexCount", "I", vertex_count).await?;
        jvm.put_field(&mut this, "version", "I", 1i32).await?;
        let value_count = (vertex_count * component_count) as usize;
        if component_size == 1 {
            let byte_data = jvm.instantiate_array("B", value_count).await?;
            jvm.put_field(&mut this, "byteData", "[B", byte_data).await?;
            jvm.put_field(&mut this, "shortData", "[S", null_ref::<Array<i16>>()).await
        } else {
            let short_data = jvm.instantiate_array("S", value_count).await?;
            jvm.put_field(&mut this, "byteData", "[B", null_ref::<Array<i8>>()).await?;
            jvm.put_field(&mut this, "shortData", "[S", short_data).await
        }
    }

    pub(super) async fn get_component_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentCount", "I").await
    }

    pub(super) async fn get_component_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "componentSize", "I").await
    }

    pub(super) async fn get_vertex_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "vertexCount", "I").await
    }

    pub(super) async fn set_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::set_array_data(jvm, this, first, count, Some(values), None).await
    }

    pub(super) async fn set_shorts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i16>>,
    ) -> Result<()> {
        Self::set_array_data(jvm, this, first, count, None, Some(values)).await
    }

    pub(super) async fn get_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i8>>,
    ) -> Result<()> {
        Self::get_array_data(jvm, this, first, count, Some(values), None).await
    }

    pub(super) async fn get_shorts(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        values: ClassInstanceRef<Array<i16>>,
    ) -> Result<()> {
        Self::get_array_data(jvm, this, first, count, None, Some(values)).await
    }

    pub(super) async fn set_array_data(
        jvm: &Jvm,
        mut this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        bytes: Option<ClassInstanceRef<Array<i8>>>,
        shorts: Option<ClassInstanceRef<Array<i16>>>,
    ) -> Result<()> {
        let (component_size, component_count, vertex_count, value_count, offset) = Self::checked_range(jvm, &this, first, count).await?;
        if let Some(values) = bytes {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.set").await);
            }
            if component_size != 1 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "byte vertex data").await);
            }
            let mut data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "byteData", "[B").await?;
            if data.is_null() {
                data = jvm.instantiate_array("B", (vertex_count * component_count) as usize).await?.into();
            }
            let values: Vec<i8> = jvm.load_array(&values, 0, value_count).await?;
            jvm.store_array(&mut data, offset, values).await?;
            jvm.put_field(&mut this, "byteData", "[B", data).await?;
        } else if let Some(values) = shorts {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.set").await);
            }
            if component_size != 2 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "short vertex data").await);
            }
            let mut data: ClassInstanceRef<Array<i16>> = jvm.get_field(&this, "shortData", "[S").await?;
            if data.is_null() {
                data = jvm.instantiate_array("S", (vertex_count * component_count) as usize).await?.into();
            }
            let values: Vec<i16> = jvm.load_array(&values, 0, value_count).await?;
            jvm.store_array(&mut data, offset, values).await?;
            jvm.put_field(&mut this, "shortData", "[S", data).await?;
        } else {
            return Ok(());
        }
        Self::bump_version(jvm, &mut this).await
    }

    pub(super) async fn bump_version(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let version = jvm.get_field::<i32>(this, "version", "I").await.unwrap_or(0).wrapping_add(1);
        jvm.put_field(this, "version", "I", version).await
    }

    pub(super) async fn get_array_data(
        jvm: &Jvm,
        this: ClassInstanceRef<Self>,
        first: i32,
        count: i32,
        bytes: Option<ClassInstanceRef<Array<i8>>>,
        shorts: Option<ClassInstanceRef<Array<i16>>>,
    ) -> Result<()> {
        let (component_size, _component_count, _vertex_count, value_count, offset) = Self::checked_range(jvm, &this, first, count).await?;
        if let Some(mut values) = bytes {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.get").await);
            }
            if component_size != 1 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "byte vertex data").await);
            }
            let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "byteData", "[B").await?;
            let data: Vec<i8> = if data.is_null() {
                vec![0; value_count]
            } else {
                jvm.load_array(&data, offset, value_count).await?
            };
            jvm.store_array(&mut values, 0, data).await
        } else if let Some(mut values) = shorts {
            if values.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "VertexArray.get").await);
            }
            if component_size != 2 || jvm.array_length(&values).await? < value_count {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "short vertex data").await);
            }
            let data: ClassInstanceRef<Array<i16>> = jvm.get_field(&this, "shortData", "[S").await?;
            let data: Vec<i16> = if data.is_null() {
                vec![0; value_count]
            } else {
                jvm.load_array(&data, offset, value_count).await?
            };
            jvm.store_array(&mut values, 0, data).await
        } else {
            Ok(())
        }
    }

    pub(super) async fn checked_range(jvm: &Jvm, this: &ClassInstanceRef<Self>, first: i32, count: i32) -> Result<(i32, i32, i32, usize, usize)> {
        let component_size: i32 = jvm.get_field(this, "componentSize", "I").await?;
        let component_count: i32 = jvm.get_field(this, "componentCount", "I").await?;
        let vertex_count: i32 = jvm.get_field(this, "vertexCount", "I").await?;
        if first < 0 || count < 0 || first.saturating_add(count) > vertex_count {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "vertex array range").await);
        }
        let value_count = (count * component_count) as usize;
        let offset = (first * component_count) as usize;
        Ok((component_size, component_count, vertex_count, value_count, offset))
    }
}
