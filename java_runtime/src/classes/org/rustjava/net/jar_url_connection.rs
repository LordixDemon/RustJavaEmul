use alloc::{string::String as RustString, string::ToString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{Object, String},
        net::URL,
        util::jar::JarFile,
    },
};

// class rustjava.net.JarURLConnection
pub struct JarURLConnection;

impl JarURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/rustjava/net/JarURLConnection",
            parent_class: Some("java/net/JarURLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("getJarFile", "()Ljava/util/jar/JarFile;", Self::get_jar_file, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("openedFiles", "Ljava/util/Hashtable;", FieldAccessFlags::STATIC)],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<clinit>()");

        let map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
        jvm.put_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<init>({:?}, {:?})", &this, &url);

        let _: () = jvm
            .invoke_special(&this, "java/net/JarURLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        Ok(())
    }

    async fn get_jar_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JarFile>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getJarFile({:?})", &this);

        let url = jvm.invoke_virtual(&this, "getJarFileURL", "()Ljava/net/URL;", ()).await?;
        let protocol = jvm.invoke_virtual(&url, "getProtocol", "()Ljava/lang/String;", ()).await?;
        let protocol = JavaLangString::to_rust_string(jvm, &protocol).await?;

        if protocol == "file" {
            let raw_name: ClassInstanceRef<String> = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;
            let raw_name = JavaLangString::to_rust_string(jvm, &raw_name).await?;
            let name = JavaLangString::from_rust_string(jvm, &normalize_file_path(&raw_name)).await?;

            let opened_files = jvm
                .get_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;")
                .await?;
            let cache: ClassInstanceRef<JarFile> = jvm
                .invoke_virtual(&opened_files, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (name.clone(),))
                .await?;

            if !cache.is_null() {
                Ok(cache)
            } else {
                let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name.clone(),)).await?;
                let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &opened_files,
                        "put",
                        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                        (name, jar_file.clone()),
                    )
                    .await?;

                Ok(jar_file.into())
            }
        } else {
            Err(jvm.exception("java/net/MalformedURLException", "unsupported protocol").await)
        }
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getInputStream({:?})", &this);

        let entry: ClassInstanceRef<String> = jvm.invoke_virtual(&this, "getEntryName", "()Ljava/lang/String;", ()).await?;
        let jar_file = jvm.invoke_virtual(&this, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;

        let jar_entry: ClassInstanceRef<JarFile> = jvm
            .invoke_virtual(&jar_file, "getJarEntry", "(Ljava/lang/String;)Ljava/util/jar/JarEntry;", (entry,))
            .await?;

        if jar_entry.is_null() {
            return Err(jvm.exception("java/io/FileNotFoundException", "entry not found").await);
        }

        let jar_input_stream = jvm
            .invoke_virtual(
                &jar_file,
                "getInputStream",
                "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                (jar_entry,),
            )
            .await?;

        Ok(jar_input_stream)
    }
}

fn normalize_file_path(path: &str) -> RustString {
    let decoded = percent_decode(path);
    let decoded = decoded.strip_prefix("file:").unwrap_or(&decoded).replace('\\', "/");
    let bytes = decoded.as_bytes();
    if bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':' {
        decoded[1..].to_string()
    } else {
        decoded
    }
}

fn percent_decode(value: &str) -> RustString {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(value.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex_value(bytes[index + 1]), hex_value(bytes[index + 2])) {
                out.push(high << 4 | low);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    RustString::from_utf8_lossy(&out).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
