use java_runtime::classes::java::lang::String;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_print_writer() -> Result<()> {
    let jvm = test_jvm().await?;

    let sw = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let pw = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (sw.clone(),)).await?;

    let hello = JavaLangString::from_rust_string(&jvm, "hello").await?;
    let world = JavaLangString::from_rust_string(&jvm, "world").await?;

    let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (hello,)).await?;
    let _: () = jvm.invoke_virtual(&pw, "println", "(Ljava/lang/String;)V", (world,)).await?;

    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&sw, "toString", "()Ljava/lang/String;", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!(result, "hello\nworld\n");

    Ok(())
}

#[tokio::test]
async fn test_print_writer_format_and_newline() -> Result<()> {
    let jvm = test_jvm().await?;

    let sw = jvm.new_class("java/io/StringWriter", "()V", ()).await?;
    let pw = jvm.new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (sw.clone(),)).await?;

    let fmt = JavaLangString::from_rust_string(&jvm, "hi %s").await?;
    let name = JavaLangString::from_rust_string(&jvm, "game").await?;
    let mut args = jvm.instantiate_array("Ljava/lang/Object;", 1).await?;
    jvm.store_array(&mut args, 0, core::iter::once(name)).await?;
    let _: ClassInstanceRef<java_runtime::classes::java::io::PrintWriter> = jvm
        .invoke_virtual(&pw, "format", "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;", (fmt, args))
        .await?;
    let _: () = jvm.invoke_virtual(&pw, "println", "()V", ()).await?;

    let bw = jvm.new_class("java/io/BufferedWriter", "(Ljava/io/Writer;)V", (sw.clone(),)).await?;
    let _: () = jvm.invoke_virtual(&bw, "newLine", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&bw, "flush", "()V", ()).await?;

    let result: ClassInstanceRef<String> = jvm.invoke_virtual(&sw, "toString", "()Ljava/lang/String;", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &result).await?;
    assert_eq!(result, "hi game\n\n");

    Ok(())
}
