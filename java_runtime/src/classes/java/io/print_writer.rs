use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{OutputStream, Writer},
        lang::{Object, String},
    },
};

// class java.io.PrintWriter
pub struct PrintWriter;

impl PrintWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;Z)V", Self::init_writer_auto_flush, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init_stream, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;Z)V", Self::init_stream_auto_flush, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write, Default::default()),
                JavaMethodProto::new("print", "(Ljava/lang/String;)V", Self::print, Default::default()),
                JavaMethodProto::new("println", "()V", Self::println_empty, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, Default::default()),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;",
                    Self::format,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/Writer;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "out", "Ljava/io/Writer;", out).await?;

        Ok(())
    }

    async fn init_writer_auto_flush(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<Writer>,
        _auto_flush: bool,
    ) -> Result<()> {
        Self::init(jvm, context, this, out).await
    }

    async fn init_stream(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        let charset = JavaLangString::from_rust_string(jvm, "UTF-8").await?;
        let writer = jvm
            .new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (out, charset),
            )
            .await?;
        Self::init(jvm, context, this, writer.into()).await
    }

    async fn init_stream_auto_flush(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        _auto_flush: bool,
    ) -> Result<()> {
        Self::init_stream(jvm, context, this, out).await
    }

    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.PrintWriter::write({:?}, {:?}, {:?}, {:?})", &this, &chars, &off, &len);

        let out = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;

        let _: i32 = jvm.invoke_virtual(&out, "write", "([CII)I", (chars, off, len)).await?;

        Ok(len)
    }

    async fn println(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintWriter::println({:?}, {:?})", &this, &string);

        let string = format!("{}\n", JavaLangString::to_rust_string(jvm, &string).await?);
        let string = JavaLangString::from_rust_string(jvm, &string).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (string,)).await?;

        Ok(())
    }

    async fn print(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (string,)).await
    }

    async fn println_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let newline = JavaLangString::from_rust_string(jvm, "\n").await?;
        jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (newline,)).await
    }

    async fn println_object(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, obj: ClassInstanceRef<Object>) -> Result<()> {
        let string = if obj.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?
        } else {
            jvm.invoke_virtual(&obj, "toString", "()Ljava/lang/String;", ()).await?
        };
        Self::println(jvm, context, this, string.into()).await
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        args: ClassInstanceRef<Array<ClassInstanceRef<Object>>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let formatted: ClassInstanceRef<String> = jvm
            .invoke_static(
                "java/lang/String",
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                (format, args),
            )
            .await?;
        let _: () = jvm.invoke_virtual(&this, "write", "(Ljava/lang/String;)V", (formatted,)).await?;
        Ok(this)
    }
}
