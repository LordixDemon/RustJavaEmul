use jvm::{JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_parse_int() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "42").await?;
    assert_eq!(
        42i32,
        jvm.invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_parse_int_invalid() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let result: Result<i32> = jvm
        .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
        .await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {result:?}");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    Ok(())
}

#[tokio::test]
async fn test_integer_compare_decode() -> Result<()> {
    let jvm = test_jvm().await?;

    let a = jvm.new_class("java/lang/Integer", "(I)V", (10,)).await?;
    let b = jvm.new_class("java/lang/Integer", "(I)V", (20,)).await?;
    let cmp: i32 = jvm.invoke_virtual(&a, "compareTo", "(Ljava/lang/Integer;)I", (b,)).await?;
    assert_eq!(cmp, -1);

    let hex = JavaLangString::from_rust_string(&jvm, "0x10").await?;
    let decoded = jvm
        .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (hex,))
        .await?;
    let value: i32 = jvm.invoke_virtual(&decoded, "intValue", "()I", ()).await?;
    assert_eq!(value, 16);

    let bit: i32 = jvm.invoke_static("java/lang/Integer", "highestOneBit", "(I)I", (0b1010,)).await?;
    assert_eq!(bit, 8);

    Ok(())
}

#[tokio::test]
async fn test_boolean_wrapper() -> Result<()> {
    let jvm = test_jvm().await?;
    let boxed = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?;
    let value: bool = jvm.invoke_virtual(&boxed, "booleanValue", "()Z", ()).await?;
    assert!(value);
    Ok(())
}
