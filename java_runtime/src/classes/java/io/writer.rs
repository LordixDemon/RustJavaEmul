use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.io.Writer
pub struct Writer;

impl Writer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Writer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write_char, Default::default()),
                JavaMethodProto::new("write", "([C)V", Self::write_char_array, Default::default()),
                JavaMethodProto::new("write", "([CII)V", Self::write_chars_void, Default::default()),
                JavaMethodProto::new("write", "([CII)I", Self::write_chars, Default::default()),
                JavaMethodProto::new("write", "(Ljava/lang/String;)V", Self::write_string, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let mut buf = jvm.instantiate_array("C", 1).await?;
        jvm.store_array(&mut buf, 0, alloc::vec![value as JavaChar]).await?;
        let _: i32 = jvm.invoke_virtual(&this, "write", "([CII)I", (buf, 0, 1)).await?;
        Ok(())
    }

    async fn write_char_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let length = jvm.array_length(&buf).await? as i32;
        let _: i32 = jvm.invoke_virtual(&this, "write", "([CII)I", (buf, 0, length)).await?;
        Ok(())
    }

    async fn write_chars_void(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<JavaChar>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        let _: i32 = jvm.invoke_virtual(&this, "write", "([CII)I", (buf, off, len)).await?;
        Ok(())
    }

    async fn write_chars(
        _: &Jvm,
        _: &mut RuntimeContext,
        _this: ClassInstanceRef<Self>,
        _buf: ClassInstanceRef<Array<JavaChar>>,
        _off: i32,
        len: i32,
    ) -> Result<i32> {
        Ok(len.max(0))
    }

    async fn write_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        tracing::debug!("java.io.Writer::write_string({:?}, {:?})", &this, &string);

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;

        let _: i32 = jvm.invoke_virtual(&this, "write", "([CII)I", (chars, 0, length as i32)).await?;

        Ok(())
    }

    async fn flush(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}
