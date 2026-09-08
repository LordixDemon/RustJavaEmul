use alloc::{format, string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::{Object, String},
    },
};

// class java.util.Properties
pub struct Properties;

impl Properties {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Properties",
            parent_class: Some("java/util/Hashtable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/util/Properties;)V", Self::init_defaults, Default::default()),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property_default,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    Self::set_property,
                    Default::default(),
                ),
                JavaMethodProto::new("load", "(Ljava/io/InputStream;)V", Self::load, Default::default()),
                JavaMethodProto::new("store", "(Ljava/io/OutputStream;Ljava/lang/String;)V", Self::store, Default::default()),
                JavaMethodProto::new("save", "(Ljava/io/OutputStream;Ljava/lang/String;)V", Self::store, Default::default()),
                JavaMethodProto::new("propertyNames", "()Ljava/util/Enumeration;", Self::property_names, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("defaults", "Ljava/util/Properties;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Properties::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Hashtable", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_defaults(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        defaults: ClassInstanceRef<Self>,
    ) -> Result<()> {
        Self::init(jvm, context, this.clone()).await?;
        jvm.put_field(&mut this, "defaults", "Ljava/util/Properties;", defaults).await
    }

    async fn get_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Properties::getProperty({:?}, {:?})", &this, &key);

        let result: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
            .await?;
        if !result.is_null() {
            return Ok(result);
        }

        let defaults: ClassInstanceRef<Self> = jvm.get_field(&this, "defaults", "Ljava/util/Properties;").await?;
        if defaults.is_null() {
            return Ok(None.into());
        }
        jvm.invoke_virtual(&defaults, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await
    }

    async fn get_property_default(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
        default: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        let value = Self::get_property(jvm, context, this, key).await?;
        if value.is_null() { Ok(default) } else { Ok(value) }
    }

    async fn set_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Properties::setProperty({:?}, {:?}, {:?})", &this, &key, &value);

        let old = jvm
            .invoke_virtual(&this, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;

        Ok(old)
    }

    async fn load(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, input: ClassInstanceRef<InputStream>) -> Result<()> {
        if input.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let mut bytes = Vec::new();
        loop {
            let next: i32 = jvm.invoke_virtual(&input, "read", "()I", ()).await?;
            if next < 0 {
                break;
            }
            bytes.push(next as u8);
        }
        let text = RustString::from_utf8_lossy(&bytes);
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }
            let Some(split) = line.find('=').or_else(|| line.find(':')) else {
                continue;
            };
            let key = line[..split].trim();
            let value = line[split + 1..].trim();
            if key.is_empty() {
                continue;
            }
            let java_key = JavaLangString::from_rust_string(jvm, key).await?;
            let java_value = JavaLangString::from_rust_string(jvm, value).await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &this,
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    (java_key, java_value),
                )
                .await?;
        }
        Ok(())
    }

    async fn store(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        output: ClassInstanceRef<OutputStream>,
        comments: ClassInstanceRef<String>,
    ) -> Result<()> {
        if output.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        if !comments.is_null() {
            let comment = JavaLangString::to_rust_string(jvm, &comments).await?;
            Self::write_bytes(jvm, &output, format!("# {comment}\n").as_bytes()).await?;
        }
        let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "keys", "()Ljava/util/Enumeration;", ()).await?;
        while jvm.invoke_virtual(&keys, "hasMoreElements", "()Z", ()).await? {
            let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "nextElement", "()Ljava/lang/Object;", ()).await?;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                .await?;
            let key_text = if key.is_null() {
                RustString::new()
            } else {
                JavaLangString::to_rust_string(jvm, &key).await?
            };
            let value_text = if value.is_null() {
                RustString::new()
            } else {
                JavaLangString::to_rust_string(jvm, &value).await?
            };
            Self::write_bytes(jvm, &output, format!("{key_text}={value_text}\n").as_bytes()).await?;
        }
        Ok(())
    }

    async fn property_names(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.invoke_virtual(&this, "keys", "()Ljava/util/Enumeration;", ()).await
    }

    async fn write_bytes(jvm: &Jvm, output: &ClassInstanceRef<OutputStream>, bytes: &[u8]) -> Result<()> {
        let mut array = jvm.instantiate_array("B", bytes.len()).await?;
        let signed: Vec<i8> = bytes.iter().copied().map(|b| b as i8).collect();
        jvm.store_array(&mut array, 0, signed).await?;
        jvm.invoke_virtual(output, "write", "([B)V", (array,)).await
    }
}
