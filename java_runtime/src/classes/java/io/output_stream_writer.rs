use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::OutputStream, lang::String},
};

pub struct OutputStreamWriter;

impl OutputStreamWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/OutputStreamWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init_stream, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/OutputStream;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        _charset: ClassInstanceRef<String>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", out).await
    }

    async fn init_stream(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        let charset = jvm::runtime::JavaLangString::from_rust_string(jvm, "UTF-8").await?;
        Self::init(jvm, context, this, out, charset.into()).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() || len <= 0 {
            return Ok(0);
        }
        let chars: alloc::vec::Vec<JavaChar> = jvm.load_array(&buf, off.max(0) as usize, len as usize).await?;
        let rust = alloc::string::String::from_utf16_lossy(&chars);
        let bytes = rust.into_bytes();
        let mut array = jvm.instantiate_array("B", bytes.len()).await?;
        let signed: alloc::vec::Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();
        jvm.store_array(&mut array, 0, signed).await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (array,)).await?;
        Ok(len)
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if !out.is_null() {
            let _: () = jvm.invoke_virtual(&out, "flush", "()V", ()).await?;
        }
        Ok(())
    }

    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::flush(jvm, context, this.clone()).await?;
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if !out.is_null() {
            let _: () = jvm.invoke_virtual(&out, "close", "()V", ()).await?;
        }
        Ok(())
    }
}
