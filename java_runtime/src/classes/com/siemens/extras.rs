#[allow(unused_imports)]
use super::*;
use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{DataInputStream, InputStream},
        lang::String,
    },
};
#[allow(unused_imports)]
use alloc::vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

pub struct FileConnection;
pub struct SiemensCommand;
pub struct SiemensImage;
pub struct SiemensManager;
pub struct SiemensPlayer;
pub struct SiemensResource;
pub struct MessagePart;
pub struct MultipartMessage;
pub struct SiemensMessageConnection;

simple_exception!(
    SiemensMediaException,
    "com/siemens/mp/media/MediaException",
    "java/lang/Exception",
    "com.siemens.mp.media.MediaException"
);

impl FileConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/io/file/FileConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "openDataInputStream",
                    "()Ljava/io/DataInputStream;",
                    Self::open_data_input_stream,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    pub(super) async fn open_data_input_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<DataInputStream>> {
        let bytes = jvm.instantiate_array("B", 0).await?;
        let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
        Ok(jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (stream,))
            .await?
            .into())
    }
}

impl SiemensCommand {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/lcdui/Command",
            parent_class: Some("javax/microedition/lcdui/Command"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/lang/String;IIC)V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        label: ClassInstanceRef<String>,
        command_type: i32,
        priority: i32,
        _key: JavaChar,
    ) -> Result<()> {
        jvm.invoke_special(
            &this,
            "javax/microedition/lcdui/Command",
            "<init>",
            "(Ljava/lang/String;II)V",
            (label, command_type, priority),
        )
        .await
    }
}

impl SiemensImage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/lcdui/Image",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "writeImageToFile",
                "(Ljavax/microedition/lcdui/Image;Ljava/lang/String;I)V",
                Self::write_image_to_file,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn write_image_to_file(
        _: &Jvm,
        _: &mut RuntimeContext,
        _image: ClassInstanceRef<crate::classes::javax::microedition::lcdui::Image>,
        _name: ClassInstanceRef<String>,
        _quality: i32,
    ) -> Result<()> {
        Ok(())
    }
}

impl SiemensPlayer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/media/Player",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getState", "()I", Self::get_state, Default::default()),
                JavaMethodProto::new("prefetch", "()V", Self::prefetch, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("stop", "()V", Self::stop, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("state", "I", Default::default())],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "state", "I", 100).await
    }
    pub(super) async fn get_state(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "state", "I").await
    }
    pub(super) async fn prefetch(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 300).await
    }
    pub(super) async fn start(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 400).await
    }
    pub(super) async fn stop(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(&mut this, "state", "I", 200).await
    }
}

impl SiemensManager {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/media/Manager",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "createPlayer",
                "(Ljava/io/InputStream;Ljava/lang/String;)Lcom/siemens/mp/media/Player;",
                Self::create_player,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn create_player(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _stream: ClassInstanceRef<InputStream>,
        _type: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<SiemensPlayer>> {
        Ok(jvm.new_class("com/siemens/mp/media/Player", "()V", ()).await?.into())
    }
}

impl SiemensResource {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/resource/Resource",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "getCenterKeyIcon",
                "(I)C",
                Self::get_center_key_icon,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn get_center_key_icon(_: &Jvm, _: &mut RuntimeContext, _id: i32) -> Result<JavaChar> {
        Ok(0)
    }
}

impl SiemensMessageConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/wireless/messaging/MessageConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

impl MessagePart {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/wireless/messaging/MessagePart",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/InputStream;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "([BLjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_bytes,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _stream: ClassInstanceRef<InputStream>,
        _mime: ClassInstanceRef<String>,
        _encoding: ClassInstanceRef<String>,
        _id: ClassInstanceRef<String>,
        _location: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    pub(super) async fn init_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _data: ClassInstanceRef<Array<i8>>,
        _mime: ClassInstanceRef<String>,
        _encoding: ClassInstanceRef<String>,
        _id: ClassInstanceRef<String>,
        _location: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}

impl MultipartMessage {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "com/siemens/mp/wireless/messaging/MultipartMessage",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "addAddress",
                    "(Ljava/lang/String;Ljava/lang/String;)Z",
                    Self::add_address,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "addMessagePart",
                    "(Lcom/siemens/mp/wireless/messaging/MessagePart;)V",
                    Self::add_message_part,
                    Default::default(),
                ),
                JavaMethodProto::new("setSubject", "(Ljava/lang/String;)V", Self::set_subject, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
    pub(super) async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
    pub(super) async fn add_address(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _type: ClassInstanceRef<String>,
        _address: ClassInstanceRef<String>,
    ) -> Result<bool> {
        Ok(true)
    }
    stub_void! {
        add_message_part(_part: ClassInstanceRef<MessagePart>);
        set_subject(_subject: ClassInstanceRef<String>);
    }
}
