use alloc::{string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, Object, String},
    },
};

// class java.lang.Class
pub struct Class;

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Class",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("isAssignableFrom", "(Ljava/lang/Class;)Z", Self::is_assignable_from, Default::default()),
                JavaMethodProto::new("isInterface", "()Z", Self::is_interface, Default::default()),
                JavaMethodProto::new("isArray", "()Z", Self::is_array, Default::default()),
                JavaMethodProto::new("isInstance", "(Ljava/lang/Object;)Z", Self::is_instance, Default::default()),
                JavaMethodProto::new("getSuperclass", "()Ljava/lang/Class;", Self::get_superclass, Default::default()),
                JavaMethodProto::new("getModifiers", "()I", Self::get_modifiers, Default::default()),
                JavaMethodProto::new("newInstance", "()Ljava/lang/Object;", Self::new_instance, Default::default()),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "forName",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    Self::for_name,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                // Stored as raw bytes instead of java/lang/String to avoid circular dependency:
                // from_rust_class -> JavaLangString::from_rust_string -> new_class("java/lang/String") -> from_rust_class -> stack overflow
                JavaFieldProto::new("nameBytes", "[B", Default::default()),
                JavaFieldProto::new("classLoader", "Ljava/lang/ClassLoader;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({:?})", &this);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let result = JavaLangString::from_rust_string(jvm, &rust_class.name().replace('/', ".")).await?;

        Ok(result.into())
    }

    async fn is_assignable_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Class::isAssignableFrom({:?}, {:?})", &this, &other);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let other_rust_class = JavaLangClass::to_rust_class(jvm, &other).await?;

        Ok(jvm.is_inherited_from(&*other_rust_class, &rust_class.name()))
    }

    async fn new_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.lang.Class::newInstance({:?})", &this);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(jvm.new_class(&rust_class.name(), "()V", ()).await?.into())
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({:?}, {:?})", &this, &name);

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let resolved = if rust_name.starts_with('/') {
            rust_name.trim_start_matches('/').to_string()
        } else {
            let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
            let class_name = rust_class.name();
            match class_name.rsplit_once('/') {
                Some((pkg, _)) => alloc::format!("{pkg}/{rust_name}"),
                None => rust_name,
            }
        };
        let name = JavaLangString::from_rust_string(jvm, &resolved).await?;

        let class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await?;

        let class_loader = if class_loader.is_null() {
            JavaLangClassLoader::get_system_class_loader(jvm).await?
        } else {
            class_loader.into()
        };

        jvm.invoke_virtual(&class_loader, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (name,))
            .await
    }

    async fn for_name(jvm: &Jvm, _context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.Class::forName({:?})", &name);

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let qualified_name = rust_name.replace('.', "/");
        match jvm.resolve_class(&qualified_name).await {
            Ok(class) => Ok(class.java_class().into()),
            Err(jvm::JavaError::JavaException(_)) => Err(jvm.exception("java/lang/ClassNotFoundException", &rust_name).await),
        }
    }

    async fn is_interface(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(rust_class.access_flags().contains(ClassAccessFlags::INTERFACE))
    }

    async fn is_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(rust_class.name().starts_with('['))
    }

    async fn is_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, obj: ClassInstanceRef<Object>) -> Result<bool> {
        if obj.is_null() {
            return Ok(false);
        }
        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(jvm.is_instance(&**obj, &rust_class.name()))
    }

    async fn get_superclass(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Class>> {
        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        if rust_class.name().starts_with('[') {
            let object = jvm.resolve_class("java/lang/Object").await?;
            return Ok(object.java_class().into());
        }
        match rust_class.super_class_name() {
            Some(super_name) => {
                let super_class = jvm.resolve_class(&super_name).await?;
                Ok(super_class.java_class().into())
            }
            None => Ok(None.into()),
        }
    }

    async fn get_modifiers(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(rust_class.access_flags().bits() as i32)
    }
}
