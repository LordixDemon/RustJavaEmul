use jvm::{Array, ClassInstanceRef, JavaError, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_mark_reset() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 5).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, &[10, 20, 30, 40, 50])?;

    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;

    let first: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(first, 10);

    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (100,)).await?;

    let second: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(second, 20);

    let _: () = jvm.invoke_virtual(&stream, "reset", "()V", ()).await?;

    let again: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(again, 20);

    Ok(())
}

#[tokio::test]
async fn test_null_buffer_is_npe() -> Result<()> {
    let jvm = test_jvm().await?;
    let null: ClassInstanceRef<Array<i8>> = None.into();
    let result = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (null,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("expected NullPointerException");
    };
    let class = exception.class_definition();
    assert_eq!(class.name(), "java/lang/NullPointerException");
    Ok(())
}

#[tokio::test]
async fn test_offset_length_window() -> Result<()> {
    let jvm = test_jvm().await?;
    let mut buffer = jvm.instantiate_array("B", 5).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, &[10, 20, 30, 40, 50])?;

    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([BII)V", (buffer, 2, 2)).await?;
    let available: i32 = jvm.invoke_virtual(&stream, "available", "()I", ()).await?;
    assert_eq!(available, 2);

    let first: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    let second: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    let third: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!((first, second, third), (30, 40, -1));

    Ok(())
}
