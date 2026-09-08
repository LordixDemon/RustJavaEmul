use std::time::Instant;

use anyhow::Context;
use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Jvm, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;
use test_utils::test_jvm;

use super::{EngineConfig, Row, mix};

pub struct JvmSuite {
    pub rows: Vec<Row>,
    pub profile_note: String,
}

pub fn run(config: &EngineConfig) -> anyhow::Result<JvmSuite> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .context("tokio runtime")?;
    runtime.block_on(run_async(config))
}

async fn run_async(config: &EngineConfig) -> anyhow::Result<JvmSuite> {
    let jvm = test_jvm().await.map_err(jvm_err)?;
    jvm_rust::set_profile_enabled(true);
    jvm_rust::reset_profile();

    let mut rows = Vec::new();
    rows.push(run_string(&jvm, config).await?);
    rows.push(run_string_buffer(&jvm, config).await?);
    rows.push(run_hashtable(&jvm, config).await?);
    rows.push(run_vector(&jvm, config).await?);
    rows.push(run_math(&jvm, config).await?);
    rows.push(run_integer(&jvm, config).await?);
    rows.push(run_arrays(&jvm, config).await?);
    rows.push(run_bytecode(&jvm, config).await?);

    let snapshot = jvm_rust::profile_snapshot();
    let profile_note = format!(
        "jvm\tinterpreter_profile\topcodes={}\tfast={}\tslow={}\tjumps={}\tinvoke_static={}\tinvoke_virtual={}\tarray_copy={}\t{}",
        snapshot.opcodes,
        snapshot.fast_opcodes,
        snapshot.slow_opcodes,
        snapshot.jumps,
        snapshot.invoke_static,
        snapshot.invoke_virtual,
        snapshot.array_copy,
        jvm_rust::method_opcode_report_and_reset(12),
    );
    Ok(JvmSuite { rows, profile_note })
}

async fn run_string(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    for index in 0..config.jvm_ops {
        let left = JavaLangString::from_rust_string(jvm, &format!("key-{index}")).await.map_err(jvm_err)?;
        let right = JavaLangString::from_rust_string(jvm, "-value").await.map_err(jvm_err)?;
        let concat: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&left, "concat", "(Ljava/lang/String;)Ljava/lang/String;", (right,))
            .await
            .map_err(jvm_err)?;
        let hash: i32 = jvm.invoke_virtual(&concat, "hashCode", "()I", ()).await.map_err(jvm_err)?;
        let needle = JavaLangString::from_rust_string(jvm, "value").await.map_err(jvm_err)?;
        let found: i32 = jvm
            .invoke_virtual(&concat, "indexOf", "(Ljava/lang/String;)I", (needle,))
            .await
            .map_err(jvm_err)?;
        let upper: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&concat, "toUpperCase", "()Ljava/lang/String;", ())
            .await
            .map_err(jvm_err)?;
        let text = JavaLangString::to_rust_string(jvm, &upper).await.map_err(jvm_err)?;
        checksum = mix(checksum, hash as u32 as u64);
        checksum = mix(checksum, found as u32 as u64);
        checksum = mix(checksum, text.len() as u64);
        ops = ops.saturating_add(4);
    }
    Ok(Row::new("jvm", "string", ops, "calls", checksum, start.elapsed()))
}

async fn run_string_buffer(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    let buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await.map_err(jvm_err)?;
    for index in 0..config.jvm_ops {
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&buffer, "append", "(I)Ljava/lang/StringBuffer;", (index as i32,))
            .await
            .map_err(jvm_err)?;
        ops = ops.saturating_add(1);
    }
    let text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ())
        .await
        .map_err(jvm_err)?;
    let rust = JavaLangString::to_rust_string(jvm, &text).await.map_err(jvm_err)?;
    checksum = mix(checksum, rust.len() as u64);
    Ok(Row::new("jvm", "string_buffer", ops, "calls", checksum, start.elapsed()))
}

async fn run_hashtable(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    let table = jvm.new_class("java/util/Hashtable", "()V", ()).await.map_err(jvm_err)?;
    let mut keys = Vec::with_capacity(config.jvm_ops as usize);
    for index in 0..config.jvm_ops {
        let key = JavaLangString::from_rust_string(jvm, &format!("k{index}")).await.map_err(jvm_err)?;
        let value = JavaLangString::from_rust_string(jvm, &format!("v{index}")).await.map_err(jvm_err)?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &table,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key.clone(), value),
            )
            .await
            .map_err(jvm_err)?;
        keys.push(key);
        ops = ops.saturating_add(1);
    }
    for key in &keys {
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&table, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
            .await
            .map_err(jvm_err)?;
        let text = JavaLangString::to_rust_string(jvm, &value).await.map_err(jvm_err)?;
        checksum = mix(checksum, text.len() as u64);
        ops = ops.saturating_add(1);
    }
    let size: i32 = jvm.invoke_virtual(&table, "size", "()I", ()).await.map_err(jvm_err)?;
    checksum = mix(checksum, size as u32 as u64);
    Ok(Row::new("jvm", "hashtable", ops, "calls", checksum, start.elapsed()))
}

async fn run_vector(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    let vector = jvm.new_class("java/util/Vector", "()V", ()).await.map_err(jvm_err)?;
    for index in 0..config.jvm_ops {
        let value = JavaLangString::from_rust_string(jvm, &format!("e{index}")).await.map_err(jvm_err)?;
        let _: () = jvm
            .invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (value,))
            .await
            .map_err(jvm_err)?;
        ops = ops.saturating_add(1);
    }
    for index in 0..config.jvm_ops {
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (index as i32,))
            .await
            .map_err(jvm_err)?;
        let text = JavaLangString::to_rust_string(jvm, &value).await.map_err(jvm_err)?;
        checksum = mix(checksum, text.len() as u64);
        ops = ops.saturating_add(1);
    }
    Ok(Row::new("jvm", "vector", ops, "calls", checksum, start.elapsed()))
}

async fn run_math(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    for index in 0..config.jvm_ops {
        let x = (index as f64 + 1.0) * 0.017;
        let sin: f64 = jvm.invoke_static("java/lang/Math", "sin", "(D)D", (x,)).await.map_err(jvm_err)?;
        let cos: f64 = jvm.invoke_static("java/lang/Math", "cos", "(D)D", (x,)).await.map_err(jvm_err)?;
        let sqrt: f64 = jvm
            .invoke_static("java/lang/Math", "sqrt", "(D)D", (x.abs() + 1.0,))
            .await
            .map_err(jvm_err)?;
        let pow: f64 = jvm
            .invoke_static("java/lang/Math", "pow", "(DD)D", (x.abs() + 1.1, 1.5))
            .await
            .map_err(jvm_err)?;
        checksum = mix(checksum, sin.to_bits());
        checksum = mix(checksum, cos.to_bits());
        checksum = mix(checksum, sqrt.to_bits());
        checksum = mix(checksum, pow.to_bits());
        ops = ops.saturating_add(4);
    }
    Ok(Row::new("jvm", "math", ops, "calls", checksum, start.elapsed()))
}

async fn run_integer(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    for index in 0..config.jvm_ops {
        let text = JavaLangString::from_rust_string(jvm, &index.to_string()).await.map_err(jvm_err)?;
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (text,))
            .await
            .map_err(jvm_err)?;
        let boxed: ClassInstanceRef<Object> = jvm
            .invoke_static("java/lang/Integer", "valueOf", "(I)Ljava/lang/Integer;", (parsed,))
            .await
            .map_err(jvm_err)?;
        let back: i32 = jvm.invoke_virtual(&boxed, "intValue", "()I", ()).await.map_err(jvm_err)?;
        checksum = mix(checksum, back as u32 as u64);
        ops = ops.saturating_add(3);
    }
    Ok(Row::new("jvm", "integer", ops, "calls", checksum, start.elapsed()))
}

async fn run_arrays(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = Instant::now();
    let length = config.jvm_ops.max(16) as usize;
    let mut src = jvm.instantiate_array("I", length).await.map_err(jvm_err)?;
    let values: Vec<i32> = (0..length as i32).collect();
    jvm.store_array(&mut src, 0, values.clone()).await.map_err(jvm_err)?;
    let dst = jvm.instantiate_array("I", length).await.map_err(jvm_err)?;
    let _: () = jvm
        .invoke_static(
            "java/lang/System",
            "arraycopy",
            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
            (src, 0, dst, 0, length as i32),
        )
        .await
        .map_err(jvm_err)?;
    checksum = mix(checksum, values.iter().fold(0u64, |acc, value| acc.wrapping_add(*value as u64)));
    ops = ops.saturating_add(length as u64);
    Ok(Row::new("jvm", "arrays", ops, "elements", checksum, start.elapsed()))
}

async fn run_bytecode(jvm: &Jvm, config: &EngineConfig) -> anyhow::Result<Row> {
    let bytes = super::bytecode::hot_class_bytes();
    let class = ClassDefinitionImpl::from_classfile(&bytes).map_err(|err| anyhow::anyhow!("EngineBenchHot classfile: {err}"))?;
    jvm.register_class(Box::new(class), None).await.map_err(jvm_err)?;

    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let start = Instant::now();
    let result: i32 = jvm
        .invoke_static(super::bytecode::CLASS_NAME, "loop", "(I)I", (config.jvm_loop as i32,))
        .await
        .map_err(jvm_err)?;
    let bits: i32 = jvm
        .invoke_static(super::bytecode::CLASS_NAME, "bits", "(I)I", (result,))
        .await
        .map_err(jvm_err)?;
    checksum = mix(checksum, result as u32 as u64);
    checksum = mix(checksum, bits as u32 as u64);
    Ok(Row::new(
        "jvm",
        "bytecode_loop",
        config.jvm_loop as u64,
        "iters",
        checksum,
        start.elapsed(),
    ))
}

fn jvm_err(err: impl core::fmt::Display) -> anyhow::Error {
    anyhow::anyhow!("{err}")
}
