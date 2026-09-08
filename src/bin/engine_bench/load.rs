use std::hint::black_box;

use classfile::ClassInfo;
use java_runtime::zip::{ZipArchive, create_deflated_zip, create_simple_zip};

use super::{EngineConfig, Row, mix};

const CLASS_SAMPLES: &[(&str, &[u8])] = &[
    ("Hello", include_bytes!("../../../test_data/Hello.class")),
    ("ControlFlow", include_bytes!("../../../test_data/ControlFlow.class")),
    ("Array", include_bytes!("../../../test_data/Array.class")),
    ("Switch", include_bytes!("../../../test_data/Switch.class")),
    ("LongDouble", include_bytes!("../../../test_data/LongDouble.class")),
    ("TypeConversion", include_bytes!("../../../test_data/TypeConversion.class")),
    ("Method", include_bytes!("../../../test_data/Method.class")),
    ("Field", include_bytes!("../../../test_data/Field.class")),
    ("SuperClass", include_bytes!("../../../test_data/SuperClass.class")),
    ("Interface", include_bytes!("../../../test_data/Interface.class")),
    ("IntegerOverflow", include_bytes!("../../../test_data/IntegerOverflow.class")),
    ("MultiArray", include_bytes!("../../../test_data/MultiArray.class")),
    ("OddEven", include_bytes!("../../../test_data/OddEven.class")),
    ("Constants", include_bytes!("../../../test_data/Constants.class")),
];

const TEST_JAR: &[u8] = include_bytes!("../../../test_data/test.jar");

pub fn run(config: &EngineConfig) -> anyhow::Result<Vec<Row>> {
    let mut rows = vec![
        run_classfile(config),
        run_zip_store(config),
        run_zip_deflate(config)?,
        run_zip_jar(config)?,
    ];
    if let Some(path) = &config.extra_jar {
        rows.push(run_zip_path(config, path)?);
    }
    Ok(rows)
}

fn run_classfile(config: &EngineConfig) -> Row {
    let hot = super::bytecode::hot_class_bytes();
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = std::time::Instant::now();
    for _ in 0..config.frames {
        for (name, bytes) in CLASS_SAMPLES {
            let class = ClassInfo::parse(bytes).unwrap_or_else(|| panic!("failed to parse {name}"));
            checksum = mix(checksum, class.constant_pool.len() as u64);
            checksum = mix(checksum, class.methods.len() as u64);
            checksum = mix(checksum, class.fields.len() as u64);
            checksum = mix(checksum, bytes.len() as u64);
            ops = ops.saturating_add(1);
            black_box(&class);
        }
        let class = ClassInfo::parse(&hot).expect("generated EngineBenchHot must parse");
        checksum = mix(checksum, class.methods.len() as u64);
        ops = ops.saturating_add(1);
        black_box(&class);
    }
    Row::new("load", "classfile", ops, "classes", checksum, start.elapsed())
}

fn run_zip_store(config: &EngineConfig) -> Row {
    let payload = repeating_payload(config.zip_bytes);
    let archive = create_simple_zip(&[("payload.bin", payload.as_slice()), ("readme.txt", b"rustjava load bench")]);
    inflate_archive("zip_store", &archive, 1)
}

fn run_zip_deflate(config: &EngineConfig) -> anyhow::Result<Row> {
    let payload = repeating_payload(config.zip_bytes);
    let archive = create_deflated_zip(&[("payload.bin", payload.as_slice()), ("readme.txt", b"rustjava load bench")]);
    Ok(inflate_archive("zip_deflate", &archive, 1))
}

fn run_zip_jar(config: &EngineConfig) -> anyhow::Result<Row> {
    Ok(inflate_archive("zip_test_jar", TEST_JAR, config.frames.max(1)))
}

fn run_zip_path(_config: &EngineConfig, path: &std::path::Path) -> anyhow::Result<Row> {
    let bytes = std::fs::read(path)?;
    Ok(inflate_archive("zip_extra_jar", &bytes, 1))
}

fn inflate_archive(phase: &'static str, bytes: &[u8], rounds: u32) -> Row {
    let mut checksum = 0xcbf2_9ce4_8422_2325;
    let mut ops = 0u64;
    let start = std::time::Instant::now();
    for _ in 0..rounds.max(1) {
        let zip = ZipArchive::new(bytes).expect("zip must parse");
        checksum = mix(checksum, zip.len() as u64);
        for index in 0..zip.len() {
            let file = zip.by_index(index).expect("zip index");
            let extracted = file.extract().expect("zip extract");
            checksum = mix(checksum, extracted.len() as u64);
            if let Some(first) = extracted.first() {
                checksum = mix(checksum, *first as u64);
            }
            ops = ops.saturating_add(extracted.len() as u64);
            black_box(&extracted);
        }
        black_box(&zip);
    }
    Row::new("load", phase, ops, "bytes", checksum, start.elapsed())
}

fn repeating_payload(size: usize) -> Vec<u8> {
    (0..size).map(|index| (index.wrapping_mul(31) % 251) as u8).collect()
}
