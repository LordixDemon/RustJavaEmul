use alloc::{format, string::ToString, vec};

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::OutputStream,
        lang::{Object, String},
    },
};

// class java.io.PrintStream
pub struct PrintStream;

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;Z)V", Self::init_auto_flush, Default::default()),
                JavaMethodProto::new("print", "(Ljava/lang/String;)V", Self::print_string, Default::default()),
                JavaMethodProto::new("print", "(C)V", Self::print_char, Default::default()),
                JavaMethodProto::new("print", "(Ljava/lang/Object;)V", Self::print_object, Default::default()),
                JavaMethodProto::new("print", "(D)V", Self::print_double, Default::default()),
                JavaMethodProto::new("print", "(I)V", Self::print_int, Default::default()),
                JavaMethodProto::new("print", "(J)V", Self::print_long, Default::default()),
                JavaMethodProto::new("print", "(Z)V", Self::print_bool, Default::default()),
                JavaMethodProto::new("checkError", "()Z", Self::check_error, Default::default()),
                JavaMethodProto::new("println", "()V", Self::println_empty, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, Default::default()),
                JavaMethodProto::new("println", "(I)V", Self::println_int, Default::default()),
                JavaMethodProto::new("println", "(J)V", Self::println_long, Default::default()),
                JavaMethodProto::new("println", "(C)V", Self::println_char, Default::default()),
                JavaMethodProto::new("println", "(B)V", Self::println_byte, Default::default()),
                JavaMethodProto::new("println", "(S)V", Self::println_short, Default::default()),
                JavaMethodProto::new("println", "(Z)V", Self::println_bool, Default::default()),
                JavaMethodProto::new("println", "(D)V", Self::println_double, Default::default()),
                JavaMethodProto::new("println", "(F)V", Self::println_float, Default::default()),
                JavaMethodProto::new("println", "([C)V", Self::println_chars, Default::default()),
                JavaMethodProto::new(
                    "printf",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;",
                    Self::printf,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::<init>({:?}, {:?})", &this, &out);

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;

        Ok(())
    }

    async fn init_auto_flush(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        _auto_flush: bool,
    ) -> Result<()> {
        Self::init(jvm, context, this, out).await
    }

    async fn write_text(jvm: &Jvm, this: &ClassInstanceRef<Self>, text: &str) -> Result<()> {
        let bytes = text.as_bytes();
        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut string_bytes).await?.write(0, bytes)?;
        jvm.invoke_virtual(this, "write", "([B)V", (string_bytes,)).await
    }

    async fn println_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, obj: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &obj);

        let result = if obj.is_null() {
            "null\n".into()
        } else {
            let string = jvm.invoke_virtual(&obj, "toString", "()Ljava/lang/String;", ()).await?;

            format!("{}\n", JavaLangString::to_rust_string(jvm, &string).await?)
        };

        let bytes = result.into_bytes();

        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut string_bytes).await?.write(0, &bytes)?;

        let _: () = jvm.invoke_virtual(&this, "write", "([B)V", (string_bytes,)).await?;

        Ok(())
    }

    async fn println_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &str);

        let result = if str.is_null() {
            "null\n".into()
        } else {
            format!("{}\n", JavaLangString::to_rust_string(jvm, &str).await?)
        };

        let bytes = result.into_bytes();

        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut string_bytes).await?.write(0, &bytes)?;

        let _: () = jvm.invoke_virtual(&this, "write", "([B)V", (string_bytes,)).await?;

        Ok(())
    }

    async fn print_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::print({:?}, {:?})", &this, &str);

        let result = if str.is_null() {
            "null".into()
        } else {
            JavaLangString::to_rust_string(jvm, &str).await?
        };

        let bytes = result.into_bytes();
        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut string_bytes).await?.write(0, &bytes)?;

        jvm.invoke_virtual(&this, "write", "([B)V", (string_bytes,)).await
    }

    async fn print_object(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, obj: ClassInstanceRef<Object>) -> Result<()> {
        let string = if obj.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?
        } else {
            jvm.invoke_virtual(&obj, "toString", "()Ljava/lang/String;", ()).await?
        };
        Self::print_string(jvm, context, this, string.into()).await
    }

    async fn check_error(_: &Jvm, _: &mut RuntimeContext, _this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn print_char(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, char: JavaChar) -> Result<()> {
        let ch = char::from_u32(char as _).unwrap_or('?');
        let java_string = JavaLangString::from_rust_string(jvm, &ch.to_string()).await?;
        Self::print_string(jvm, context, this, java_string.into()).await
    }

    async fn println_empty(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let java_string = JavaLangString::from_rust_string(jvm, "").await?;
        Self::println_string(jvm, context, this, java_string.into()).await
    }

    async fn println_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, int: i32) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &int);

        let java_string = JavaLangString::from_rust_string(jvm, &int.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, long: i64) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &long);

        let java_string = JavaLangString::from_rust_string(jvm, &long.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, char: JavaChar) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &char);

        let char = char::from_u32(char as _).unwrap();

        let java_string = JavaLangString::from_rust_string(jvm, &char.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i8) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &byte);

        let java_string = JavaLangString::from_rust_string(jvm, &byte.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, short: i16) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &short);

        let java_string = JavaLangString::from_rust_string(jvm, &short.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, bool: bool) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &bool);

        let java_string = JavaLangString::from_rust_string(jvm, &bool.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, double: f64) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &double);

        let string = format!("{double:.1}");

        let java_string = JavaLangString::from_rust_string(jvm, &string).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let java_string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await
    }

    async fn println_chars(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<()> {
        if chars.is_null() {
            Self::println_string(jvm, context, this, JavaLangString::from_rust_string(jvm, "null").await?.into()).await
        } else {
            let length = jvm.array_length(&chars).await?;
            let values: alloc::vec::Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
            let text = alloc::string::String::from_utf16_lossy(&values);
            let java_string = JavaLangString::from_rust_string(jvm, &text).await?;
            Self::println_string(jvm, context, this, java_string.into()).await
        }
    }

    async fn print_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        Self::write_text(jvm, &this, &value.to_string()).await
    }

    async fn print_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        Self::write_text(jvm, &this, &value.to_string()).await
    }

    async fn print_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        Self::write_text(jvm, &this, &value.to_string()).await
    }

    async fn print_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        Self::write_text(jvm, &this, if value { "true" } else { "false" }).await
    }

    async fn printf(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        format: ClassInstanceRef<String>,
        args: ClassInstanceRef<Array<ClassInstanceRef<Object>>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let formatted: ClassInstanceRef<String> = jvm
            .invoke_static(
                "java/lang/String",
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                (format, args),
            )
            .await?;
        let _: () = jvm.invoke_virtual(&this, "print", "(Ljava/lang/String;)V", (formatted,)).await?;
        Ok(this)
    }
}
