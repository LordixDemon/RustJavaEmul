use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    FileType, RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor, InputStream},
        lang::String,
    },
};

// class java.io.FileInputStream
pub struct FileInputStream;

impl FileInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_path, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/FileDescriptor;)V",
                    Self::init_with_file_descriptor,
                    Default::default(),
                ),
                JavaMethodProto::new("read", "()I", Self::read_byte, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read_array, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default()),
                JavaFieldProto::new("in", "Ljava/io/InputStream;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init_with_path(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &name);
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        jvm.invoke_special(&this, "java/io/FileInputStream", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &file);

        let Some(path) = File::path(jvm, &file).await? else {
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        };

        for candidate in File::path_candidates(&path) {
            if let Ok(stat) = context.metadata(&candidate).await {
                if stat.r#type == FileType::Directory {
                    return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
                }
            }
            if let Ok(fd) = context.open(&candidate, false).await {
                let fd = FileDescriptor::from_fd(jvm, fd).await?;
                return jvm
                    .invoke_special(&this, "java/io/FileInputStream", "<init>", "(Ljava/io/FileDescriptor;)V", (fd,))
                    .await;
            }
        }

        if looks_like_classpath_resource(&path) {
            if let Some(stream) = Self::classpath_stream(jvm, &path).await? {
                let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;
                jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", stream).await?;
                return Ok(());
            }
        }

        Err(jvm.exception("java/io/FileNotFoundException", "File not found").await)
    }

    async fn classpath_stream(jvm: &Jvm, path: &str) -> Result<Option<ClassInstanceRef<InputStream>>> {
        let loader = jvm
            .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
            .await?;
        for candidate in crate::resource_path_candidates(path) {
            let name = JavaLangString::from_rust_string(jvm, &candidate).await?;
            let stream: ClassInstanceRef<InputStream> = jvm
                .invoke_virtual(&loader, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (name,))
                .await?;
            if !stream.is_null() {
                return Ok(Some(stream));
            }
        }
        Ok(None)
    }

    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        fd: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &fd);

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn resource(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        jvm.get_field(this, "in", "Ljava/io/InputStream;").await
    }

    async fn available(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::available({:?})", &this);

        let resource = Self::resource(jvm, &this).await?;
        if !resource.is_null() {
            return jvm.invoke_virtual(&resource, "available", "()I", ()).await;
        }

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let Ok(stat) = rust_file.metadata().await else {
            return Ok(0);
        };
        let tell = rust_file.tell().await.unwrap_or(0);

        let available = stat.size.saturating_sub(tell);

        Ok(available as _)
    }

    async fn read_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({this:?}, {buf:?}, {offset:?}, {length:?})");

        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }

        let resource = Self::resource(jvm, &this).await?;
        if !resource.is_null() {
            return jvm.invoke_virtual(&resource, "read", "([BII)I", (buf, offset, length)).await;
        }

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut rust_buf = vec![0; length.max(0) as usize];
        let read = match rust_file.read(&mut rust_buf).await {
            Ok(read) => read,
            Err(_) => return Err(jvm.exception("java/io/IOException", "read failed").await),
        };
        if read == 0 {
            return Ok(-1);
        }

        jvm.store_array(&mut buf, offset.max(0) as _, cast_vec::<u8, i8>(rust_buf)).await?;

        Ok(read as _)
    }

    async fn read_byte(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({:?})", &this);

        let resource = Self::resource(jvm, &this).await?;
        if !resource.is_null() {
            return jvm.invoke_virtual(&resource, "read", "()I", ()).await;
        }

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut buf = [0; 1];
        let read = match rust_file.read(&mut buf).await {
            Ok(read) => read,
            Err(_) => return Err(jvm.exception("java/io/IOException", "read failed").await),
        };
        if read == 0 {
            return Ok(-1);
        }

        Ok(buf[0] as i32)
    }

    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::close({:?})", &this);

        let resource = Self::resource(jvm, &this).await?;
        if !resource.is_null() {
            return jvm.invoke_virtual(&resource, "close", "()V", ()).await;
        }

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        FileDescriptor::close(jvm, context, fd).await?;

        Ok(())
    }
}

fn looks_like_classpath_resource(path: &str) -> bool {
    let unix = path.replace('\\', "/");
    let trimmed = unix.trim_start_matches('/');
    if trimmed.is_empty() || trimmed == "." || trimmed == ".." || trimmed.starts_with("./") || trimmed.starts_with("../") {
        return false;
    }
    if trimmed.ends_with(".jar") || trimmed.ends_with(".rustjar") {
        return false;
    }
    let bytes = trimmed.as_bytes();
    if bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
        return false;
    }
    true
}
