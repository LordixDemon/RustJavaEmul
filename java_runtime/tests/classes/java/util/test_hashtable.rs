use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_hashmap() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;

    let test_key = JavaLangString::from_rust_string(&jvm, "testKey").await?;
    let test_value = JavaLangString::from_rust_string(&jvm, "testValue").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (test_key.clone(), test_value),
        )
        .await?;

    let value = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
        .await?;

    let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value_string, "testValue");

    let test_key_second = JavaLangString::from_rust_string(&jvm, "testKey").await?;

    let value = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key_second.clone(),))
        .await?;

    let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value_string, "testValue");

    let value = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
        .await?;

    let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value_string, "testValue");

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key,))
        .await?;

    assert!(value.is_null());
    Ok(())
}

#[tokio::test]
async fn test_hashtable_size_keys() -> Result<()> {
    let jvm = test_jvm().await?;
    let table = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&table, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let empty: bool = jvm.invoke_virtual(&table, "isEmpty", "()Z", ()).await?;
    assert!(empty);

    let key = JavaLangString::from_rust_string(&jvm, "k").await?;
    let value = JavaLangString::from_rust_string(&jvm, "v").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    let size: i32 = jvm.invoke_virtual(&table, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&table, "keys", "()Ljava/util/Enumeration;", ()).await?;
    let has: bool = jvm.invoke_virtual(&keys, "hasMoreElements", "()Z", ()).await?;
    assert!(has);
    Ok(())
}

#[tokio::test]
async fn test_hashtable_values() -> Result<()> {
    let jvm = test_jvm().await?;
    let table = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "k").await?;
    let value = JavaLangString::from_rust_string(&jvm, "v").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&table, "values", "()Ljava/util/Collection;", ()).await?;
    let size: i32 = jvm.invoke_virtual(&values, "size", "()I", ()).await?;
    assert_eq!(size, 1);
    Ok(())
}
