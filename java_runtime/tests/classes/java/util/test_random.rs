use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_random() -> Result<()> {
    let jvm = test_jvm().await?;

    let seed = 42i64;
    let random = jvm.new_class("java/util/Random", "(J)V", (seed,)).await?;

    let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
    assert_eq!(next, -1170105035);

    let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
    assert_eq!(next, 234785527);

    let random = jvm.new_class("java/util/Random", "(J)V", (seed,)).await?;
    let bounded: i32 = jvm.invoke_virtual(&random, "nextInt", "(I)I", (10,)).await?;
    assert_eq!(bounded, 0);

    let bounded: i32 = jvm.invoke_virtual(&random, "nextInt", "(I)I", (7,)).await?;
    assert_eq!(bounded, 5);

    Ok(())
}

#[tokio::test]
async fn test_next_gaussian() -> Result<()> {
    let jvm = test_jvm().await?;
    let random = jvm.new_class("java/util/Random", "(J)V", (1i64,)).await?;
    let first: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    let second: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    assert!(first.is_finite());
    assert!(second.is_finite());
    assert_ne!(first, second);

    Ok(())
}
