use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_default_handler_and_parse() -> Result<()> {
    let jvm = test_jvm().await?;
    let handler = jvm.new_class("org/xml/sax/helpers/DefaultHandler", "()V", ()).await?;
    let parser = jvm.new_class("javax/xml/parsers/SAXParser", "()V", ()).await?;
    let data = jvm.instantiate_array("B", 0).await?;
    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let _: () = jvm
        .invoke_virtual(
            &parser,
            "parse",
            "(Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;)V",
            (stream, handler),
        )
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_input_source_stream_ctor() -> Result<()> {
    let jvm = test_jvm().await?;
    let data = jvm.instantiate_array("B", 0).await?;
    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let source = jvm.new_class("org/xml/sax/InputSource", "(Ljava/io/InputStream;)V", (stream,)).await?;
    let handler = jvm.new_class("org/xml/sax/helpers/DefaultHandler", "()V", ()).await?;
    let parser = jvm.new_class("javax/xml/parsers/SAXParser", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &parser,
            "parse",
            "(Lorg/xml/sax/InputSource;Lorg/xml/sax/helpers/DefaultHandler;)V",
            (source, handler),
        )
        .await?;
    Ok(())
}
