#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::String,
    },
};
#[allow(unused_imports)]
use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

impl FileConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/file/FileConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["javax/microedition/io/StreamConnection"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("exists", "()Z", Self::exists, Default::default()),
                JavaMethodProto::new("isDirectory", "()Z", Self::is_directory, Default::default()),
                JavaMethodProto::new("isOpen", "()Z", Self::is_open, Default::default()),
                JavaMethodProto::new("fileSize", "()J", Self::file_size, Default::default()),
                JavaMethodProto::new("directorySize", "(Z)J", Self::directory_size, Default::default()),
                JavaMethodProto::new("canRead", "()Z", Self::can_read, Default::default()),
                JavaMethodProto::new("canWrite", "()Z", Self::can_write, Default::default()),
                JavaMethodProto::new("isHidden", "()Z", Self::is_hidden, Default::default()),
                JavaMethodProto::new("setReadable", "(Z)V", Self::set_readable, Default::default()),
                JavaMethodProto::new("setWritable", "(Z)V", Self::set_writable, Default::default()),
                JavaMethodProto::new("setHidden", "(Z)V", Self::set_hidden, Default::default()),
                JavaMethodProto::new("list", "()Ljava/util/Enumeration;", Self::list, Default::default()),
                JavaMethodProto::new(
                    "list",
                    "(Ljava/lang/String;Z)Ljava/util/Enumeration;",
                    Self::list_filter,
                    Default::default(),
                ),
                JavaMethodProto::new("mkdir", "()V", Self::mkdir, Default::default()),
                JavaMethodProto::new("create", "()V", Self::create, Default::default()),
                JavaMethodProto::new("delete", "()V", Self::delete, Default::default()),
                JavaMethodProto::new("rename", "(Ljava/lang/String;)V", Self::rename, Default::default()),
                JavaMethodProto::new("truncate", "(J)V", Self::truncate, Default::default()),
                JavaMethodProto::new(
                    "setFileConnection",
                    "(Ljava/lang/String;)V",
                    Self::set_file_connection,
                    Default::default(),
                ),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("getPath", "()Ljava/lang/String;", Self::get_path, Default::default()),
                JavaMethodProto::new("getURL", "()Ljava/lang/String;", Self::get_url, Default::default()),
                JavaMethodProto::new("lastModified", "()J", Self::last_modified, Default::default()),
                JavaMethodProto::new("availableSize", "()J", Self::available_size, Default::default()),
                JavaMethodProto::new("totalSize", "()J", Self::total_size, Default::default()),
                JavaMethodProto::new("usedSize", "()J", Self::used_size, Default::default()),
                JavaMethodProto::new("openInputStream", "()Ljava/io/InputStream;", Self::open_input, Default::default()),
                JavaMethodProto::new("openOutputStream", "()Ljava/io/OutputStream;", Self::open_output, Default::default()),
                JavaMethodProto::new(
                    "openOutputStream",
                    "(J)Ljava/io/OutputStream;",
                    Self::open_output_offset,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "openDataInputStream",
                    "()Ljava/io/DataInputStream;",
                    Self::open_data_in,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "openDataOutputStream",
                    "()Ljava/io/DataOutputStream;",
                    Self::open_data_out,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("url", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("open", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<String>,
        _mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", url).await?;
        jvm.put_field(&mut this, "open", "Z", true).await
    }

    pub(super) async fn path(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustString> {
        let url: ClassInstanceRef<String> = jvm.get_field(this, "url", "Ljava/lang/String;").await?;
        let rust = JavaLangString::to_rust_string(jvm, &url).await?;
        Ok(rust.trim_start_matches("file://").trim_start_matches("file:").to_string())
    }

    pub(super) async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "open", "Z", false).await
    }
    pub(super) async fn exists(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let path = Self::path(jvm, &this).await?;
        Ok(context.metadata(&path).await.is_ok())
    }
    pub(super) async fn is_directory(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let path = Self::path(jvm, &this).await?;
        match context.metadata(&path).await {
            Ok(stat) => Ok(stat.r#type == crate::FileType::Directory),
            Err(_) => Ok(false),
        }
    }
    pub(super) async fn is_open(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "open", "Z").await
    }
    pub(super) async fn file_size(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let path = Self::path(jvm, &this).await?;
        Ok(context.metadata(&path).await.map(|s| s.size as i64).unwrap_or(0))
    }
    pub(super) async fn directory_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _include_sub: bool) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn can_read(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(true)
    }
    pub(super) async fn can_write(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(true)
    }
    pub(super) async fn is_hidden(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }
    pub(super) async fn set_readable(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _v: bool) -> Result<()> {
        Ok(())
    }
    pub(super) async fn set_writable(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _v: bool) -> Result<()> {
        Ok(())
    }
    pub(super) async fn set_hidden(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _v: bool) -> Result<()> {
        Ok(())
    }
    pub(super) async fn list(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        Self::list_filter(jvm, context, this, ClassInstanceRef::new(None), false).await
    }
    pub(super) async fn list_filter(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _filter: ClassInstanceRef<String>,
        _include_hidden: bool,
    ) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        let path = Self::path(jvm, &this).await?;
        let names = context.list_directory(&path);
        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
        for name in names {
            let s = JavaLangString::from_rust_string(jvm, &name).await?;
            let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (s,)).await?;
        }
        jvm.invoke_virtual(&vector, "elements", "()Ljava/util/Enumeration;", ()).await
    }
    pub(super) async fn mkdir(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn create(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let path = Self::path(jvm, &this).await?;
        let _ = context.open(&path, true).await;
        Ok(())
    }
    pub(super) async fn delete(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let path = Self::path(jvm, &this).await?;
        let _ = context.unlink(&path).await;
        Ok(())
    }
    pub(super) async fn rename(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _name: ClassInstanceRef<String>) -> Result<()> {
        Ok(())
    }
    pub(super) async fn truncate(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>, _size: i64) -> Result<()> {
        Ok(())
    }
    pub(super) async fn set_file_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "url", "Ljava/lang/String;", name).await
    }
    pub(super) async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let path = Self::path(jvm, &this).await?;
        let name = path.rsplit(['/', '\\']).next().unwrap_or(&path);
        Ok(JavaLangString::from_rust_string(jvm, name).await?.into())
    }
    pub(super) async fn get_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let path = Self::path(jvm, &this).await?;
        Ok(JavaLangString::from_rust_string(jvm, &path).await?.into())
    }
    pub(super) async fn get_url(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "url", "Ljava/lang/String;").await
    }
    pub(super) async fn last_modified(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn available_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(64 * 1024 * 1024)
    }
    pub(super) async fn total_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(128 * 1024 * 1024)
    }
    pub(super) async fn used_size(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(0)
    }
    pub(super) async fn open_input(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        let path = Self::path(jvm, &this).await?;
        let path = JavaLangString::from_rust_string(jvm, &path).await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
        Ok(jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?.into())
    }
    pub(super) async fn open_output(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<OutputStream>> {
        Self::open_output_offset(jvm, context, this, 0).await
    }
    pub(super) async fn open_output_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _offset: i64,
    ) -> Result<ClassInstanceRef<OutputStream>> {
        let path = Self::path(jvm, &this).await?;
        let path = JavaLangString::from_rust_string(jvm, &path).await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
        Ok(jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;)V", (file,)).await?.into())
    }
    pub(super) async fn open_data_in(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataInputStream>> {
        let stream = Self::open_input(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
            .await?
            .into())
    }
    pub(super) async fn open_data_out(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::io::DataOutputStream>> {
        let stream = Self::open_output(jvm, context, this).await?;
        Ok(jvm
            .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream,))
            .await?
            .into())
    }
}

impl FileSystemRegistry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/file/FileSystemRegistry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("listRoots", "()Ljava/util/Enumeration;", Self::list_roots, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "addFileSystemListener",
                    "(Ljavax/microedition/io/file/FileSystemListener;)Z",
                    Self::add_listener,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "removeFileSystemListener",
                    "(Ljavax/microedition/io/file/FileSystemListener;)Z",
                    Self::remove_listener,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    pub(super) async fn list_roots(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<crate::classes::java::lang::Object>> {
        let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let root = JavaLangString::from_rust_string(jvm, "/").await?;
        let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (root,)).await?;
        jvm.invoke_virtual(&vector, "elements", "()Ljava/util/Enumeration;", ()).await
    }
    pub(super) async fn add_listener(_: &Jvm, _: &mut RuntimeContext, _listener: ClassInstanceRef<FileSystemListener>) -> Result<bool> {
        Ok(true)
    }
    pub(super) async fn remove_listener(_: &Jvm, _: &mut RuntimeContext, _listener: ClassInstanceRef<FileSystemListener>) -> Result<bool> {
        Ok(true)
    }
}

impl FileSystemListener {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/io/file/FileSystemListener",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract("rootChanged", "(ILjava/lang/String;)V", Default::default())],
            fields: vec![
                JavaFieldProto::new("ROOT_ADDED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ROOT_REMOVED", "I", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
