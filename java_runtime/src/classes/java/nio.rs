use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

pub struct Buffer;
pub struct ByteBuffer;

impl Buffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/nio/Buffer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("position", "(I)Ljava/nio/Buffer;", Self::position, Default::default()),
                JavaMethodProto::new("rewind", "()Ljava/nio/Buffer;", Self::rewind, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("position", "I", Default::default())],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn position(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pos: i32) -> Result<ClassInstanceRef<Self>> {
        jvm.put_field(&mut this, "position", "I", pos.max(0)).await?;
        Ok(this)
    }

    async fn rewind(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        jvm.put_field(&mut this, "position", "I", 0).await?;
        Ok(this)
    }
}

impl ByteBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/nio/ByteBuffer",
            parent_class: Some("java/nio/Buffer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "allocateDirect",
                    "(I)Ljava/nio/ByteBuffer;",
                    Self::allocate_direct,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("position", "(I)Ljava/nio/Buffer;", Self::position, Default::default()),
                JavaMethodProto::new("putInt", "(I)Ljava/nio/ByteBuffer;", Self::put_int, Default::default()),
                JavaMethodProto::new("putShort", "(S)Ljava/nio/ByteBuffer;", Self::put_short, Default::default()),
                JavaMethodProto::new("rewind", "()Ljava/nio/Buffer;", Self::rewind, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("data", "[B", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/nio/Buffer", "<init>", "()V", ()).await?;
        let array = jvm.instantiate_array("B", capacity.max(0) as _).await?;
        jvm.put_field(&mut this, "data", "[B", array).await
    }

    async fn allocate_direct(jvm: &Jvm, _: &mut RuntimeContext, capacity: i32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/nio/ByteBuffer", "(I)V", (capacity,)).await?.into())
    }

    async fn position(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, pos: i32) -> Result<ClassInstanceRef<Buffer>> {
        jvm.invoke_special(&this, "java/nio/Buffer", "position", "(I)Ljava/nio/Buffer;", (pos,))
            .await
    }

    async fn rewind(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Buffer>> {
        jvm.invoke_special(&this, "java/nio/Buffer", "rewind", "()Ljava/nio/Buffer;", ()).await
    }

    async fn put_int(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        let pos: i32 = jvm.get_field(&this, "position", "I").await?;
        let mut data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "data", "[B").await?;
        let bytes = value.to_be_bytes();
        jvm.store_array(&mut data, pos as _, vec![bytes[0] as i8, bytes[1] as i8, bytes[2] as i8, bytes[3] as i8])
            .await?;
        jvm.put_field(&mut this, "position", "I", pos + 4).await?;
        Ok(this)
    }

    async fn put_short(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i16) -> Result<ClassInstanceRef<Self>> {
        let pos: i32 = jvm.get_field(&this, "position", "I").await?;
        let mut data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "data", "[B").await?;
        let bytes = value.to_be_bytes();
        jvm.store_array(&mut data, pos as _, vec![bytes[0] as i8, bytes[1] as i8]).await?;
        jvm.put_field(&mut this, "position", "I", pos + 2).await?;
        Ok(this)
    }
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![Buffer, ByteBuffer]
}
