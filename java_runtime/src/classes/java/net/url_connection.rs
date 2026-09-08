use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        net::URL,
    },
};

// class java.net.URLConnection
pub struct URLConnection;

impl URLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
                JavaMethodProto::new("getOutputStream", "()Ljava/io/OutputStream;", Self::get_output_stream, Default::default()),
                JavaMethodProto::new("setDoOutput", "(Z)V", Self::set_do_output, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("url", "Ljava/net/URL;", Default::default()),
                JavaFieldProto::new("doOutput", "Z", Default::default()),
                JavaFieldProto::new("outputStream", "Ljava/io/OutputStream;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?})", &this, &url,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URL::getInputStream({:?})", &this);

        Err(jvm.exception("java/io/UnknownServiceException", "unsupported").await)
    }

    async fn set_do_output(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, do_output: bool) -> Result<()> {
        jvm.put_field(&mut this, "doOutput", "Z", do_output).await
    }

    async fn get_output_stream(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<OutputStream>> {
        let existing: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "outputStream", "Ljava/io/OutputStream;").await?;
        if !existing.is_null() {
            return Ok(existing);
        }
        let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
        jvm.put_field(&mut this, "outputStream", "Ljava/io/OutputStream;", stream.clone()).await?;
        Ok(stream.into())
    }
}
