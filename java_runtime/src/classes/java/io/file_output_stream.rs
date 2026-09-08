use alloc::vec;

use bytemuck::cast_slice;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
};

// class java.io.FileOutputStream
pub struct FileOutputStream;

impl FileOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_path, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Z)V", Self::init_with_path_append, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/File;Z)V", Self::init_with_file_append, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/FileDescriptor;)V",
                    Self::init_with_file_descriptor,
                    Default::default(),
                ),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes_offset, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init_with_path(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        jvm.invoke_special(&this, "java/io/FileOutputStream", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init_with_path_append(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        _append: bool,
    ) -> Result<()> {
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        jvm.invoke_special(&this, "java/io/FileOutputStream", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init_with_file_append(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        _append: bool,
    ) -> Result<()> {
        Self::init(jvm, context, this, file).await
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({:?}, {:?})", &this, &file);

        let Some(path) = File::path(jvm, &file).await? else {
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        };

        for candidate in File::path_candidates(&path) {
            if let Ok(fd) = context.open(&candidate, true).await {
                let fd = FileDescriptor::from_fd(jvm, fd).await?;
                return jvm
                    .invoke_special(&this, "java/io/FileOutputStream", "<init>", "(Ljava/io/FileDescriptor;)V", (fd,))
                    .await;
            }
        }

        Err(jvm.exception("java/io/FileNotFoundException", "File not found").await)
    }

    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_descriptor: ClassInstanceRef<File>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({:?}, {:?})", &this, &file_descriptor);

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", file_descriptor).await?;

        Ok(())
    }

    async fn write_bytes_offset(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.io.FileOutputStream::write({:?}, {:?}, {:?}, {:?})",
            &this,
            &buffer,
            &offset,
            &length
        );

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, context, fd).await?;

        let mut buf = vec![0; length.max(0) as usize];
        if jvm.array_raw_buffer(&buffer).await?.read(offset.max(0) as _, &mut buf).is_err() {
            return Err(jvm.exception("java/io/IOException", "write failed").await);
        }

        if file.write(cast_slice(&buf)).await.is_err() {
            return Err(jvm.exception("java/io/IOException", "write failed").await);
        }

        Ok(())
    }

    async fn write(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i32) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::write({:?}, {:?})", &this, &byte);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, context, fd).await?;

        if file.write(&[byte as u8]).await.is_err() {
            return Err(jvm.exception("java/io/IOException", "write failed").await);
        }

        Ok(())
    }

    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::close({:?})", &this);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        FileDescriptor::close(jvm, context, fd).await?;

        Ok(())
    }
}
