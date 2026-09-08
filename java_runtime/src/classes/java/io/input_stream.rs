use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.io.InputStream
pub struct InputStream;

impl InputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/InputStream",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read_offset_length, Default::default()),
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_byte, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("skip", "(J)J", Self::skip, Default::default()),
                JavaMethodProto::new("mark", "(I)V", Self::mark, Default::default()),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.InputStream::read({:?}, {:?})", &this, &b);

        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let array_length = jvm.array_length(&b).await? as i32;

        jvm.invoke_virtual(&this, "read", "([BII)I", (b, 0, array_length)).await
    }

    async fn read_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.InputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "").await);
        }
        let length = jvm.array_length(&b).await? as i32;
        if off < 0 || len < 0 || off + len > length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "").await);
        }
        if len == 0 {
            return Ok(0);
        }

        let first: i32 = jvm.invoke_virtual(&this, "read", "()I", ()).await?;
        if first < 0 {
            return Ok(-1);
        }
        jvm.store_array(&mut b, off as _, core::iter::once(first as i8)).await?;

        let mut n = 1;
        while n < len {
            let next: i32 = match jvm.invoke_virtual(&this, "read", "()I", ()).await {
                Ok(value) => value,
                Err(_) => break,
            };
            if next < 0 {
                break;
            }
            jvm.store_array(&mut b, (off + n) as _, core::iter::once(next as i8)).await?;
            n += 1;
        }
        Ok(n)
    }

    async fn read_byte(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(-1)
    }

    async fn available(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.InputStream::skip({:?}, {:?})", &this, n);

        if n <= 0 {
            return Ok(0);
        }

        let scratch_size = n.min(4096);
        let scratch = jvm.instantiate_array("B", scratch_size as _).await?;

        let mut remaining = n;
        while remaining > 0 {
            let len_to_read = remaining.min(scratch_size) as i32;
            let read: i32 = jvm.invoke_virtual(&this, "read", "([BII)I", (scratch.clone(), 0, len_to_read)).await?;
            if read <= 0 {
                break;
            }

            remaining -= read as i64;
        }

        Ok(n - remaining)
    }

    async fn mark(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.InputStream::mark({:?}, {:?})", &this, readlimit);

        Ok(())
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::reset({:?})", &this);

        Err(jvm.exception("java/io/IOException", "reset not supported").await)
    }
}
