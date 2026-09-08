use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::Writer};

pub struct BufferedWriter;

impl BufferedWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write, Default::default()),
                JavaMethodProto::new("newLine", "()V", Self::new_line, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/Writer;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "out", "Ljava/io/Writer;", out).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        if out.is_null() {
            return Ok(0);
        }
        jvm.invoke_virtual(&out, "write", "([CII)I", (buf, off, len)).await
    }

    async fn new_line(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let newline = JavaLangString::from_rust_string(jvm, "\n").await?;
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (newline,)).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        if !out.is_null() {
            let _: () = jvm.invoke_virtual(&out, "flush", "()V", ()).await?;
        }
        Ok(())
    }

    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        Self::flush(jvm, context, this.clone()).await?;
        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        if !out.is_null() {
            let _: () = jvm.invoke_virtual(&out, "close", "()V", ()).await?;
        }
        Ok(())
    }
}
