use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, Reader},
        lang::{Object, String},
    },
};

pub struct Attributes;
pub struct DefaultHandler;
pub struct InputSource;

simple_exception!(
    SAXException,
    "org/xml/sax/SAXException",
    "java/lang/Exception",
    "org.xml.sax.SAXException"
);
simple_exception!(
    SAXParseException,
    "org/xml/sax/SAXParseException",
    "org/xml/sax/SAXException",
    "org.xml.sax.SAXParseException"
);

simple_object!(Attributes, "org/xml/sax/Attributes");

impl InputSource {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/xml/sax/InputSource",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init_stream, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/Reader;)V", Self::init_reader, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_system_id, Default::default()),
                JavaMethodProto::new("setByteStream", "(Ljava/io/InputStream;)V", Self::set_byte_stream, Default::default()),
                JavaMethodProto::new("getByteStream", "()Ljava/io/InputStream;", Self::get_byte_stream, Default::default()),
                JavaMethodProto::new(
                    "setCharacterStream",
                    "(Ljava/io/Reader;)V",
                    Self::set_character_stream,
                    Default::default(),
                ),
                JavaMethodProto::new("getCharacterStream", "()Ljava/io/Reader;", Self::get_character_stream, Default::default()),
                JavaMethodProto::new("setSystemId", "(Ljava/lang/String;)V", Self::set_system_id, Default::default()),
                JavaMethodProto::new("getSystemId", "()Ljava/lang/String;", Self::get_system_id, Default::default()),
                JavaMethodProto::new("setEncoding", "(Ljava/lang/String;)V", Self::set_encoding, Default::default()),
                JavaMethodProto::new("getEncoding", "()Ljava/lang/String;", Self::get_encoding, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("byteStream", "Ljava/io/InputStream;", Default::default()),
                JavaFieldProto::new("characterStream", "Ljava/io/Reader;", Default::default()),
                JavaFieldProto::new("systemId", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("encoding", "Ljava/lang/String;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn init_stream(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, stream: ClassInstanceRef<InputStream>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "byteStream", "Ljava/io/InputStream;", stream).await
    }

    async fn init_reader(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, reader: ClassInstanceRef<Reader>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "characterStream", "Ljava/io/Reader;", reader).await
    }

    async fn init_system_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, system_id: ClassInstanceRef<String>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "systemId", "Ljava/lang/String;", system_id).await
    }

    async fn set_byte_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        stream: ClassInstanceRef<InputStream>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "byteStream", "Ljava/io/InputStream;", stream).await
    }

    async fn get_byte_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        jvm.get_field(&this, "byteStream", "Ljava/io/InputStream;").await
    }

    async fn set_character_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        reader: ClassInstanceRef<Reader>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "characterStream", "Ljava/io/Reader;", reader).await
    }

    async fn get_character_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Reader>> {
        jvm.get_field(&this, "characterStream", "Ljava/io/Reader;").await
    }

    async fn set_system_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, system_id: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "systemId", "Ljava/lang/String;", system_id).await
    }

    async fn get_system_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "systemId", "Ljava/lang/String;").await
    }

    async fn set_encoding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, encoding: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "encoding", "Ljava/lang/String;", encoding).await
    }

    async fn get_encoding(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "encoding", "Ljava/lang/String;").await
    }
}

impl DefaultHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/xml/sax/helpers/DefaultHandler",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("startDocument", "()V", Self::start_document, Default::default()),
                JavaMethodProto::new("endDocument", "()V", Self::end_document, Default::default()),
                JavaMethodProto::new(
                    "startElement",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/xml/sax/Attributes;)V",
                    Self::start_element,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "endElement",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::end_element,
                    Default::default(),
                ),
                JavaMethodProto::new("characters", "([CII)V", Self::characters, Default::default()),
                JavaMethodProto::new("ignorableWhitespace", "([CII)V", Self::ignorable_whitespace, Default::default()),
                JavaMethodProto::new(
                    "processingInstruction",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::processing_instruction,
                    Default::default(),
                ),
                JavaMethodProto::new("skippedEntity", "(Ljava/lang/String;)V", Self::skipped_entity, Default::default()),
                JavaMethodProto::new("error", "(Lorg/xml/sax/SAXParseException;)V", Self::error, Default::default()),
                JavaMethodProto::new("fatalError", "(Lorg/xml/sax/SAXParseException;)V", Self::fatal_error, Default::default()),
                JavaMethodProto::new("warning", "(Lorg/xml/sax/SAXParseException;)V", Self::warning, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    stub_void! {
        start_document();
        end_document();
        start_element(_uri: ClassInstanceRef<String>, _local: ClassInstanceRef<String>, _qname: ClassInstanceRef<String>, _attributes: ClassInstanceRef<Attributes>);
        end_element(_uri: ClassInstanceRef<String>, _local: ClassInstanceRef<String>, _qname: ClassInstanceRef<String>);
        characters(_ch: ClassInstanceRef<Object>, _start: i32, _length: i32);
        ignorable_whitespace(_ch: ClassInstanceRef<Object>, _start: i32, _length: i32);
        processing_instruction(_target: ClassInstanceRef<String>, _data: ClassInstanceRef<String>);
        skipped_entity(_name: ClassInstanceRef<String>);
        error(_exception: ClassInstanceRef<SAXParseException>);
        fatal_error(_exception: ClassInstanceRef<SAXParseException>);
        warning(_exception: ClassInstanceRef<SAXParseException>);
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Attributes, DefaultHandler, InputSource, SAXException, SAXParseException]
}
