use alloc::{vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::{io::InputStream, lang::String},
        javax::microedition::media::Player,
    },
};

// class javax.microedition.media.Manager
pub struct Manager;

impl Manager {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/microedition/media/Manager",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "createPlayer",
                    "(Ljava/io/InputStream;Ljava/lang/String;)Ljavax/microedition/media/Player;",
                    Self::create_player,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getSupportedContentTypes",
                    "(Ljava/lang/String;)[Ljava/lang/String;",
                    Self::get_supported_content_types,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getSupportedProtocols",
                    "(Ljava/lang/String;)[Ljava/lang/String;",
                    Self::get_supported_protocols,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn create_player(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        input: ClassInstanceRef<InputStream>,
        content_type: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Player>> {
        tracing::debug!("javax.microedition.media.Manager::createPlayer({input:?}, {content_type:?})");

        Ok(jvm.new_class("javax/microedition/media/Player", "()V", ()).await?.into())
    }

    async fn get_supported_content_types(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _protocol: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        Self::string_array(jvm, &["audio/midi", "audio/x-midi", "audio/amr", "audio/wav", "audio/mpeg"]).await
    }

    async fn get_supported_protocols(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _content_type: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        Self::string_array(jvm, &["file", "http", "resource"]).await
    }

    async fn string_array(jvm: &Jvm, values: &[&str]) -> Result<ClassInstanceRef<Array<ClassInstanceRef<String>>>> {
        let mut array = jvm.instantiate_array("Ljava/lang/String;", values.len()).await?;
        let mut strings = Vec::with_capacity(values.len());
        for value in values {
            strings.push(JavaLangString::from_rust_string(jvm, value).await?);
        }
        jvm.store_array(&mut array, 0, strings).await?;
        Ok(array.into())
    }
}
