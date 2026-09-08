use alloc::{format, string::String as AllocString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{FileType, RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.io.File
pub struct File;

impl File {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/File",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/File;Ljava/lang/String;)V",
                    Self::init_parent_file,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_parent_string,
                    Default::default(),
                ),
                JavaMethodProto::new("getPath", "()Ljava/lang/String;", Self::get_path, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("getParent", "()Ljava/lang/String;", Self::get_parent, Default::default()),
                JavaMethodProto::new("getParentFile", "()Ljava/io/File;", Self::get_parent_file, Default::default()),
                JavaMethodProto::new("getAbsolutePath", "()Ljava/lang/String;", Self::get_absolute_path, Default::default()),
                JavaMethodProto::new("getAbsoluteFile", "()Ljava/io/File;", Self::get_absolute_file, Default::default()),
                JavaMethodProto::new("isAbsolute", "()Z", Self::is_absolute, Default::default()),
                JavaMethodProto::new("exists", "()Z", Self::exists, Default::default()),
                JavaMethodProto::new("isDirectory", "()Z", Self::is_directory, Default::default()),
                JavaMethodProto::new("isFile", "()Z", Self::is_file, Default::default()),
                JavaMethodProto::new("delete", "()Z", Self::delete, Default::default()),
                JavaMethodProto::new("length", "()J", Self::length, Default::default()),
                JavaMethodProto::new("list", "()[Ljava/lang/String;", Self::list, Default::default()),
                JavaMethodProto::new("listRoots", "()[Ljava/io/File;", Self::list_roots, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("path", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("separator", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("separatorChar", "C", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("pathSeparator", "Ljava/lang/String;", FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let separator = JavaLangString::from_rust_string(jvm, "/").await?;
        let path_separator = JavaLangString::from_rust_string(jvm, ":").await?;
        jvm.put_static_field("java/io/File", "separator", "Ljava/lang/String;", separator).await?;
        jvm.put_static_field("java/io/File", "separatorChar", "C", b'/' as JavaChar).await?;
        jvm.put_static_field("java/io/File", "pathSeparator", "Ljava/lang/String;", path_separator)
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pathname: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.File::<init>({:?}, {:?})", &this, &pathname);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "path", "Ljava/lang/String;", pathname).await?;

        Ok(())
    }

    async fn init_parent_file(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        parent: ClassInstanceRef<Self>,
        child: ClassInstanceRef<String>,
    ) -> Result<()> {
        let parent_path = if parent.is_null() {
            AllocString::new()
        } else {
            Self::path(jvm, &parent).await?.unwrap_or_default()
        };
        let child_path = if child.is_null() {
            AllocString::new()
        } else {
            JavaLangString::to_rust_string(jvm, &child).await?
        };
        let joined = JavaLangString::from_rust_string(jvm, &join_file_path(&parent_path, &child_path)).await?;
        Self::init(jvm, context, this, joined.into()).await
    }

    async fn init_parent_string(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        parent: ClassInstanceRef<String>,
        child: ClassInstanceRef<String>,
    ) -> Result<()> {
        let parent_path = if parent.is_null() {
            AllocString::new()
        } else {
            JavaLangString::to_rust_string(jvm, &parent).await?
        };
        let child_path = if child.is_null() {
            AllocString::new()
        } else {
            JavaLangString::to_rust_string(jvm, &child).await?
        };
        let joined = JavaLangString::from_rust_string(jvm, &join_file_path(&parent_path, &child_path)).await?;
        Self::init(jvm, context, this, joined.into()).await
    }

    async fn get_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.File::getPath({:?})", &this);

        jvm.get_field(&this, "path", "Ljava/lang/String;").await
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let path = Self::path(jvm, &this).await?.unwrap_or_default();
        let name = file_name(&path);
        Ok(JavaLangString::from_rust_string(jvm, name).await?.into())
    }

    async fn get_parent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(None.into());
        };
        match parent_path(&path) {
            Some(parent) => Ok(JavaLangString::from_rust_string(jvm, &parent).await?.into()),
            None => Ok(None.into()),
        }
    }

    async fn get_parent_file(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let parent: ClassInstanceRef<String> = Self::get_parent(jvm, context, this).await?;
        if parent.is_null() {
            return Ok(None.into());
        }
        Ok(jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (parent,)).await?.into())
    }

    async fn get_absolute_path(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        Self::get_path(jvm, context, this).await
    }

    async fn get_absolute_file(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let path = Self::get_absolute_path(jvm, context, this).await?;
        Ok(jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?.into())
    }

    async fn is_absolute(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(Self::path(jvm, &this).await?.map(|path| is_absolute_path(&path)).unwrap_or(false))
    }

    async fn exists(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::exists({:?})", &this);

        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(false);
        };

        for candidate in Self::path_candidates(&path) {
            if context.metadata(&candidate).await.is_ok() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn is_directory(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isDirectory({:?})", &this);

        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(false);
        };

        for candidate in Self::path_candidates(&path) {
            if let Ok(stat) = context.metadata(&candidate).await {
                return Ok(stat.r#type == FileType::Directory);
            }
        }
        Ok(false)
    }

    async fn is_file(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isFile({:?})", &this);

        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(false);
        };

        for candidate in Self::path_candidates(&path) {
            if let Ok(stat) = context.metadata(&candidate).await {
                return Ok(stat.r#type == FileType::File);
            }
        }
        Ok(false)
    }

    async fn delete(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::delete({:?})", &this);

        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(false);
        };

        for candidate in Self::path_candidates(&path) {
            if context.unlink(&candidate).await.is_ok() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.File::length({:?})", &this);

        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(0);
        };

        for candidate in Self::path_candidates(&path) {
            if let Ok(stat) = context.metadata(&candidate).await {
                return Ok(stat.size as i64);
            }
        }
        Ok(0)
    }

    async fn list(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        if !Self::is_directory(jvm, context, this.clone()).await? {
            return Ok(None.into());
        }
        let Some(path) = Self::path(jvm, &this).await? else {
            return Ok(None.into());
        };
        let names = context.list_directory(&path);
        let mut array = jvm.instantiate_array("Ljava/lang/String;", names.len()).await?;
        for (index, name) in names.into_iter().enumerate() {
            let java_name = JavaLangString::from_rust_string(jvm, &name).await?;
            jvm.store_array(&mut array, index, core::iter::once(java_name)).await?;
        }
        Ok(array.into())
    }

    async fn list_roots(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<ClassInstanceRef<Self>>>> {
        let root_path = JavaLangString::from_rust_string(jvm, "/").await?;
        let root = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (root_path,)).await?;
        let mut array = jvm.instantiate_array("Ljava/io/File;", 1).await?;
        jvm.store_array(&mut array, 0, core::iter::once(root)).await?;
        Ok(array.into())
    }

    pub(crate) async fn path(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Option<alloc::string::String>> {
        let path: ClassInstanceRef<String> = jvm.invoke_virtual(this, "getPath", "()Ljava/lang/String;", ()).await?;
        if path.is_null() {
            return Ok(None);
        }

        JavaLangString::to_rust_string(jvm, &path).await.map(Some)
    }

    pub(crate) fn path_candidates(path: &str) -> Vec<AllocString> {
        crate::filesystem_path_candidates(path)
    }
}

fn is_separator(ch: char) -> bool {
    ch == '/' || ch == '\\'
}

fn join_file_path(parent: &str, child: &str) -> AllocString {
    if parent.is_empty() {
        return child.into();
    }
    if child.is_empty() {
        return parent.into();
    }
    if parent.ends_with(is_separator) {
        format!("{parent}{child}")
    } else {
        format!("{parent}/{child}")
    }
}

fn file_name(path: &str) -> &str {
    let trimmed = path.trim_end_matches(is_separator);
    if trimmed.is_empty() {
        return path;
    }
    trimmed.rsplit(is_separator).next().unwrap_or(trimmed)
}

fn parent_path(path: &str) -> Option<AllocString> {
    let trimmed = path.trim_end_matches(is_separator);
    if trimmed.is_empty() || trimmed == "/" || (trimmed.len() == 2 && trimmed.as_bytes().get(1) == Some(&b':')) {
        return None;
    }
    match trimmed.rfind(is_separator) {
        Some(0) => Some("/".into()),
        Some(index) => Some(trimmed[..index].into()),
        None => None,
    }
}

fn is_absolute_path(path: &str) -> bool {
    path.starts_with('/') || path.starts_with('\\') || (path.len() >= 2 && path.as_bytes()[1] == b':')
}
