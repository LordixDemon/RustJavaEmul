use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::InputStream, lang::Object},
};

pub struct SAXParser;
pub struct SAXParserFactory;

simple_exception!(
    ParserConfigurationException,
    "javax/xml/parsers/ParserConfigurationException",
    "java/lang/Exception",
    "javax.xml.parsers.ParserConfigurationException"
);

impl SAXParser {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/xml/parsers/SAXParser",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "parse",
                    "(Lorg/xml/sax/InputSource;Lorg/xml/sax/helpers/DefaultHandler;)V",
                    Self::parse,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "parse",
                    "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;)V",
                    Self::parse_stream,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn parse(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<Object>,
        handler: ClassInstanceRef<Object>,
    ) -> Result<()> {
        if source.is_null() {
            return Self::finish_parse(jvm, handler).await;
        }
        let stream: ClassInstanceRef<InputStream> = jvm.invoke_virtual(&source, "getByteStream", "()Ljava/io/InputStream;", ()).await?;
        Self::parse_stream(jvm, context, this, stream, handler).await
    }

    async fn parse_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<InputStream>,
        handler: ClassInstanceRef<Object>,
    ) -> Result<()> {
        if !source.is_null() {
            let buf = jvm.instantiate_array("B", 4096).await?;
            loop {
                let read: i32 = jvm.invoke_virtual(&source, "read", "([B)I", (buf.clone(),)).await?;
                if read < 0 {
                    break;
                }
            }
        }
        Self::finish_parse(jvm, handler).await
    }

    async fn finish_parse(jvm: &Jvm, handler: ClassInstanceRef<Object>) -> Result<()> {
        if handler.is_null() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "handler").await);
        }
        let _: () = jvm.invoke_virtual(&handler, "startDocument", "()V", ()).await?;
        jvm.invoke_virtual(&handler, "endDocument", "()V", ()).await
    }
}

impl SAXParserFactory {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "javax/xml/parsers/SAXParserFactory",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "newInstance",
                    "()Ljavax/xml/parsers/SAXParserFactory;",
                    Self::new_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "newSAXParser",
                    "()Ljavax/xml/parsers/SAXParser;",
                    Self::new_sax_parser,
                    Default::default(),
                ),
                JavaMethodProto::new("setNamespaceAware", "(Z)V", Self::set_namespace_aware, Default::default()),
                JavaMethodProto::new("setValidating", "(Z)V", Self::set_validating, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    async fn new_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("javax/xml/parsers/SAXParserFactory", "()V", ()).await?.into())
    }
    async fn new_sax_parser(jvm: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<SAXParser>> {
        Ok(jvm.new_class("javax/xml/parsers/SAXParser", "()V", ()).await?.into())
    }
    stub_void! {
        set_namespace_aware(_aware: bool);
        set_validating(_validating: bool);
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![ParserConfigurationException, SAXParser, SAXParserFactory]
}
