use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, bail};
use jvm::runtime::JavaLangString;
use rust_java::{StartType, create_jvm_with_screen, invoke_entrypoint, java_error_to_anyhow};

use super::{EngineConfig, Row};

struct Capture {
    buf: Arc<Mutex<Vec<u8>>>,
}

impl Write for Capture {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.lock().expect("stdout capture").extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn run(config: &EngineConfig) -> anyhow::Result<Vec<Row>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .context("tokio runtime")?;
    runtime.block_on(run_async(config))
}

async fn run_async(config: &EngineConfig) -> anyhow::Result<Vec<Row>> {
    let jar_path = config.midlet.clone().unwrap_or_else(default_jar_path);
    let bytes = std::fs::read(&jar_path).with_context(|| format!("read {}", jar_path.display()))?;
    let output = Arc::new(Mutex::new(Vec::new()));
    let start_type = StartType::JarBytes {
        name: Path::new("RustJavaBench.jar"),
        bytes: &bytes,
    };
    let screen = Some((config.screen_width.max(32) as usize, config.screen_height.max(32) as usize));
    let (jvm, runtime) = create_jvm_with_screen(Capture { buf: output.clone() }, &start_type, &[], screen).await?;

    set_property(&jvm, "rustjava.bench.ms", &config.jar_ms.to_string()).await?;
    set_property(&jvm, "rustjava.bench.autoexit", "1").await?;
    if let Ok(path) = std::env::var("RUSTJAVA_BENCH_OUT") {
        set_property(&jvm, "rustjava.bench.out", &path).await?;
    }

    if let Err(err) = invoke_entrypoint(&jvm, &start_type, &[] as &[&str]).await {
        runtime.abort_spawned();
        let stdout = stdout_text(&output);
        let err = java_error_to_anyhow(&jvm, err).await;
        return Err(anyhow::anyhow!("RustJavaBench failed: {err}\n{stdout}"));
    }
    runtime.abort_spawned();
    tokio::task::yield_now().await;

    let stdout = stdout_text(&output);
    let rows = parse_rows(&stdout);
    if rows.is_empty() {
        bail!("RustJavaBench printed no RUSTJAVA_BENCH score rows\n{stdout}");
    }
    Ok(rows)
}

async fn set_property(jvm: &jvm::Jvm, key: &str, value: &str) -> anyhow::Result<()> {
    let key = JavaLangString::from_rust_string(jvm, key).await.map_err(display_err)?;
    let value = JavaLangString::from_rust_string(jvm, value).await.map_err(display_err)?;
    let _: jvm::ClassInstanceRef<java_runtime::classes::java::lang::Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key, value),
        )
        .await
        .map_err(display_err)?;
    Ok(())
}

fn parse_rows(stdout: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for line in stdout.lines() {
        let Some(rest) = line.strip_prefix("RUSTJAVA_BENCH\t") else {
            continue;
        };
        let parts: Vec<&str> = rest.split('\t').collect();
        if parts.len() != 8 {
            continue;
        }
        if parts[0] == "meta" {
            continue;
        }
        let ops = parts[2].parse().unwrap_or(0);
        let checksum = u64::from_str_radix(parts[4], 16).unwrap_or(0);
        let total_ms: f64 = parts[5].parse().unwrap_or(0.0);
        rows.push(Row::new(
            intern_suite(parts[0]),
            intern_phase(parts[1]),
            ops,
            intern_unit(parts[3]),
            checksum,
            Duration::from_secs_f64((total_ms / 1000.0).max(0.0)),
        ));
    }
    rows
}

fn intern_suite(value: &str) -> &'static str {
    match value {
        "cpu" => "cpu",
        "gfx" => "gfx",
        "game" => "game",
        "m3g" => "m3g",
        "v3" => "v3",
        _ => "jar",
    }
}

fn intern_phase(value: &str) -> &'static str {
    match value {
        "int_add" => "int_add",
        "long_mul" => "long_mul",
        "float_math" => "float_math",
        "object_alloc" => "object_alloc",
        "string" => "string",
        "hashtable" => "hashtable",
        "vector" => "vector",
        "arraycopy" => "arraycopy",
        "fill_rect" => "fill_rect",
        "draw_line" => "draw_line",
        "fill_triangle" => "fill_triangle",
        "draw_rgb" => "draw_rgb",
        "draw_string" => "draw_string",
        "draw_image" => "draw_image",
        "draw_region" => "draw_region",
        "sprite" => "sprite",
        "transform" => "transform",
        "affine" => "affine",
        _ => "phase",
    }
}

fn intern_unit(value: &str) -> &'static str {
    match value {
        "ops" => "ops",
        "calls" => "calls",
        "elements" => "elements",
        _ => "skip",
    }
}

fn stdout_text(output: &Arc<Mutex<Vec<u8>>>) -> String {
    String::from_utf8_lossy(&output.lock().expect("stdout capture")).into_owned()
}

fn default_jar_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/jar/dist/RustJavaBench.jar")
}

fn display_err(err: impl core::fmt::Display) -> anyhow::Error {
    anyhow::anyhow!("{err}")
}
