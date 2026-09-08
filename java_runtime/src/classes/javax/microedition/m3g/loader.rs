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

use super::file_loader::M3gFileLoader;
impl Loader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/m3g/Loader",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "load",
                    "(Ljava/lang/String;)[Ljavax/microedition/m3g/Object3D;",
                    Self::load_from_string,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "load",
                    "([BI)[Ljavax/microedition/m3g/Object3D;",
                    Self::load_from_bytes,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn load_from_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        name: ClassInstanceRef<JavaString>,
    ) -> Result<ClassInstanceRef<Object3DArray>> {
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Loader.load(null)").await);
        }

        let resource_name = JavaLangString::to_rust_string(jvm, &name).await?;
        tracing::info!(target: "rustjava_m3g", "m3g.loader.load resource={resource_name:?}");
        let objects = Self::load_resource_objects(jvm, context, &resource_name, &mut Vec::new()).await?;
        Self::object_array(jvm, objects).await
    }

    pub(super) async fn load_from_bytes(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
    ) -> Result<ClassInstanceRef<Object3DArray>> {
        if data.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Loader.load(null, offset)").await);
        }
        if offset < 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "negative offset").await);
        }

        let length = jvm.array_length(&data).await?;
        if offset as usize >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "offset outside data").await);
        }

        let bytes_i8: Vec<i8> = jvm.load_array(&data, offset as usize, length - offset as usize).await?;
        let bytes: Vec<u8> = bytes_i8.into_iter().map(|b| b as u8).collect();
        let objects = Self::load_bytes_objects(jvm, context, "ByteArray", &bytes, &mut Vec::new()).await?;
        Self::object_array(jvm, objects).await
    }

    #[async_recursion::async_recursion]
    pub(super) async fn load_resource_objects(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        resource_name: &str,
        history: &mut Vec<RustString>,
    ) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        let normalized = resource_name.trim_start_matches('/').to_string();
        if history.iter().any(|name| name == &normalized) {
            return Err(jvm.exception("java/io/IOException", "M3G reference loop").await);
        }
        history.push(normalized.clone());
        let bytes = Self::read_resource(jvm, &normalized).await?;
        let result = Self::load_bytes_objects(jvm, context, &normalized, &bytes, history).await;
        history.pop();
        result
    }

    pub(super) async fn load_bytes_objects(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        resource_name: &str,
        bytes: &[u8],
        history: &mut Vec<RustString>,
    ) -> Result<Vec<ClassInstanceRef<Object3D>>> {
        if bytes.starts_with(M3G_IDENTIFIER) {
            return M3gFileLoader::new(jvm, context, resource_name, history).parse(bytes).await;
        }

        if bytes.starts_with(PNG_IDENTIFIER) || bytes.starts_with(JPEG_IDENTIFIER) {
            let image = if resource_name == "ByteArray" {
                let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
                let signed: Vec<i8> = bytes.iter().map(|byte| *byte as i8).collect();
                jvm.store_array(&mut byte_array, 0, signed).await?;
                let image: ClassInstanceRef<Image> = jvm
                    .invoke_static(
                        "javax/microedition/lcdui/Image",
                        "createImage",
                        "([BII)Ljavax/microedition/lcdui/Image;",
                        (byte_array, 0, bytes.len() as i32),
                    )
                    .await?;
                image
            } else {
                let name = JavaLangString::from_rust_string(jvm, resource_name).await?;
                jvm.invoke_static(
                    "javax/microedition/lcdui/Image",
                    "createImage",
                    "(Ljava/lang/String;)Ljavax/microedition/lcdui/Image;",
                    (name,),
                )
                .await?
            };

            let image2d: ClassInstanceRef<Image2D> = jvm
                .new_class(
                    "javax/microedition/m3g/Image2D",
                    "(ILjava/lang/Object;)V",
                    (Image2D::RGBA, cast_ref::<Image, Object>(&image)),
                )
                .await?
                .into();
            return Ok(vec![cast_ref(&image2d)]);
        }

        Err(jvm
            .exception("java/io/IOException", &format!("M3G resource not recognized: {resource_name}"))
            .await)
    }

    pub(super) async fn read_resource(jvm: &Jvm, resource_name: &str) -> Result<Vec<u8>> {
        let class_loader = jvm
            .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
            .await?;
        let stream: ClassInstanceRef<InputStream> = jvm
            .invoke_virtual(
                &class_loader,
                "getResourceAsStream",
                "(Ljava/lang/String;)Ljava/io/InputStream;",
                (JavaLangString::from_rust_string(jvm, resource_name).await?,),
            )
            .await?;

        if stream.is_null() {
            return Err(jvm
                .exception("java/io/IOException", &format!("resource not found: {resource_name}"))
                .await);
        }

        JavaIoInputStream::read_until_end(jvm, &stream).await
    }

    pub(super) async fn object_array(jvm: &Jvm, objects: Vec<ClassInstanceRef<Object3D>>) -> Result<ClassInstanceRef<Object3DArray>> {
        let mut array = jvm.instantiate_array("Ljavax/microedition/m3g/Object3D;", objects.len()).await?;
        jvm.store_array(&mut array, 0, objects).await?;
        Ok(array.into())
    }
}
