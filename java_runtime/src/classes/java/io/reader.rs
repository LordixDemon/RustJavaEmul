use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.io.Reader
pub struct Reader;

impl Reader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Reader",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_char, Default::default()),
                JavaMethodProto::new("read", "([C)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "([CII)I", Self::read_offset_length, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Reader::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let buf = jvm.instantiate_array("C", 1).await?;
        let n: i32 = jvm.invoke_virtual(&this, "read", "([CII)I", (buf.clone(), 0, 1)).await?;
        if n <= 0 {
            return Ok(-1);
        }
        let ch: JavaChar = jvm.load_array(&buf, 0, 1).await?.into_iter().next().unwrap();
        Ok(ch as i32)
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<JavaChar>) -> Result<i32> {
        tracing::debug!("java.io.Reader::read({:?}, {:?})", &this, &buf);

        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }

        let len = jvm.array_length(&buf).await? as i32;
        let result = jvm.invoke_virtual(&this, "read", "([CII)I", (buf, 0, len)).await?;

        Ok(result)
    }

    async fn read_offset_length(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _buf: ClassInstanceRef<JavaChar>,
        _off: i32,
        _len: i32,
    ) -> Result<i32> {
        Ok(-1)
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}
