use std::{env, path::PathBuf, time::Duration};

use anyhow::{Context, anyhow, bail};
use java_runtime::classes::{
    com::mascotcapsule::micro3d::v3::benchmark::{V3Benchmark, V3BenchmarkConfig},
    javax::microedition::{
        LcdUiBenchmark, LcdUiBenchmarkConfig, LcdUiBenchmarkRun,
        m3g::benchmark::{M3gBenchmark, M3gBenchmarkConfig},
    },
};

mod bytecode;
#[cfg(not(target_arch = "wasm32"))]
mod jar;
#[cfg(not(target_arch = "wasm32"))]
mod jvm;
mod load;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Suite {
    V3,
    M3g,
    LcdUi,
    Load,
    Jvm,
    Jar,
}

#[derive(Clone, Debug)]
struct EngineConfig {
    screen_width: i32,
    screen_height: i32,
    grid_size: usize,
    frames: u32,
    texture_size: usize,
    jvm_ops: u32,
    jvm_loop: u32,
    zip_bytes: usize,
    jar_ms: u32,
    extra_jar: Option<PathBuf>,
    midlet: Option<PathBuf>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            screen_width: 240,
            screen_height: 320,
            grid_size: 18,
            frames: 120,
            texture_size: 64,
            jvm_ops: 1_500,
            jvm_loop: 2_000_000,
            zip_bytes: 256 * 1024,
            jar_ms: 400,
            extra_jar: None,
            midlet: None,
        }
    }
}

struct Row {
    suite: &'static str,
    phase: &'static str,
    ops: u64,
    unit: &'static str,
    checksum: u64,
    duration: Duration,
}

impl Row {
    fn new(suite: &'static str, phase: &'static str, ops: u64, unit: &'static str, checksum: u64, duration: Duration) -> Self {
        Self {
            suite,
            phase,
            ops,
            unit,
            checksum,
            duration,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let Some((config, suites)) = parse_args()? else {
        print_help();
        return Ok(());
    };

    println!(
        "engine_bench\tversion={}\tos={}\tarch={}\twidth={}\theight={}\tgrid={}\tframes={}\ttexture={}\tjvm_ops={}\tjvm_loop={}\tzip_bytes={}\tjar_ms={}\tsuites={}",
        env!("CARGO_PKG_VERSION"),
        env::consts::OS,
        env::consts::ARCH,
        config.screen_width,
        config.screen_height,
        config.grid_size,
        config.frames,
        config.texture_size,
        config.jvm_ops,
        config.jvm_loop,
        config.zip_bytes,
        config.jar_ms,
        suites.iter().map(suite_name).collect::<Vec<_>>().join(","),
    );
    println!("suite\tphase\tops\tunit\tchecksum\ttotal_ms\tms_per_op\tops_per_sec");

    let mut notes = Vec::new();
    for suite in suites {
        match suite {
            Suite::V3 => print_rows(run_v3(&config)),
            Suite::M3g => print_rows(run_m3g(&config)),
            Suite::LcdUi => print_rows(run_lcdui(&config)),
            Suite::Load => print_rows(load::run(&config)?),
            Suite::Jvm => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let report = jvm::run(&config)?;
                    print_rows(report.rows);
                    notes.push(report.profile_note);
                }
                #[cfg(target_arch = "wasm32")]
                {
                    println!("jvm\tskipped\t0\tnone\t0\t0.000\t0.0000\t0");
                }
            }
            Suite::Jar => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    print_rows(jar::run(&config)?);
                }
                #[cfg(target_arch = "wasm32")]
                {
                    println!("jar\tskipped\t0\tnone\t0\t0.000\t0.0000\t0");
                }
            }
        }
    }
    for note in notes {
        println!("{note}");
    }
    Ok(())
}

fn run_v3(config: &EngineConfig) -> Vec<Row> {
    let benchmark = V3Benchmark::new(V3BenchmarkConfig {
        screen_width: config.screen_width,
        screen_height: config.screen_height,
        grid_size: config.grid_size,
        frames: config.frames,
        texture_size: config.texture_size,
    });
    let prepared = benchmark.prepare_frame(0);
    vec![
        measure_v3("geometry", || benchmark.run_geometry()),
        measure_v3("raster", || benchmark.run_raster(&prepared)),
        measure_v3("full_frame", || benchmark.run_full_frame()),
    ]
}

fn measure_v3(phase: &'static str, run: impl FnOnce() -> java_runtime::classes::com::mascotcapsule::micro3d::v3::benchmark::V3BenchmarkRun) -> Row {
    let start = std::time::Instant::now();
    let result = run();
    Row::new("v3", phase, result.triangles, "triangles", result.checksum, start.elapsed())
}

fn run_m3g(config: &EngineConfig) -> Vec<Row> {
    let benchmark = M3gBenchmark::new(M3gBenchmarkConfig {
        screen_width: config.screen_width,
        screen_height: config.screen_height,
        grid_size: config.grid_size,
        frames: config.frames,
        texture_size: config.texture_size,
    });
    vec![
        measure_m3g("math", || benchmark.run_math()),
        measure_m3g("lighting", || benchmark.run_lighting()),
        measure_m3g("texture", || benchmark.run_texture()),
        measure_m3g("clip", || benchmark.run_clip()),
        measure_m3g("raster_flat", || benchmark.run_raster_flat()),
        measure_m3g("raster_textured", || benchmark.run_raster_textured()),
        measure_m3g("raster_blend", || benchmark.run_raster_blend()),
        measure_m3g("raster_fog", || benchmark.run_raster_fog()),
        measure_m3g("full_frame", || benchmark.run_full_frame()),
    ]
}

fn measure_m3g(phase: &'static str, run: impl FnOnce() -> java_runtime::classes::javax::microedition::m3g::benchmark::M3gBenchmarkRun) -> Row {
    let start = std::time::Instant::now();
    let result = run();
    Row::new("m3g", phase, result.ops, "ops", result.checksum, start.elapsed())
}

fn run_lcdui(config: &EngineConfig) -> Vec<Row> {
    let benchmark = LcdUiBenchmark::new(LcdUiBenchmarkConfig {
        screen_width: config.screen_width,
        screen_height: config.screen_height,
        frames: config.frames,
        sprite_size: 48,
    });
    vec![
        measure_lcdui("fill", || benchmark.run_fill()),
        measure_lcdui("blit_opaque", || benchmark.run_blit_opaque()),
        measure_lcdui("blit_alpha", || benchmark.run_blit_alpha()),
        measure_lcdui("text", || benchmark.run_text()),
        measure_lcdui("transform", || benchmark.run_transform()),
        measure_lcdui("triangle", || benchmark.run_triangle()),
        measure_lcdui("line", || benchmark.run_line()),
        measure_lcdui("full_frame", || benchmark.run_full_frame()),
    ]
}

fn measure_lcdui(phase: &'static str, run: impl FnOnce() -> LcdUiBenchmarkRun) -> Row {
    let start = std::time::Instant::now();
    let result = run();
    Row::new("lcdui", phase, result.ops, "pixels", result.checksum, start.elapsed())
}

fn print_rows(rows: Vec<Row>) {
    for row in rows {
        let seconds = row.duration.as_secs_f64().max(f64::MIN_POSITIVE);
        let total_ms = seconds * 1000.0;
        let ops = row.ops.max(1) as f64;
        println!(
            "{}\t{}\t{}\t{}\t{:016x}\t{:.3}\t{:.4}\t{:.0}",
            row.suite,
            row.phase,
            row.ops,
            row.unit,
            row.checksum,
            total_ms,
            total_ms / ops,
            row.ops as f64 / seconds,
        );
    }
}

fn parse_args() -> anyhow::Result<Option<(EngineConfig, Vec<Suite>)>> {
    let mut config = EngineConfig::default();
    let mut suites = Vec::new();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--quick" => {
                config.frames = 30;
                config.grid_size = 12;
                config.jvm_ops = 250;
                config.jvm_loop = 200_000;
                config.zip_bytes = 64 * 1024;
                config.jar_ms = 150;
            }
            "--heavy" => {
                config.frames = 180;
                config.grid_size = 28;
                config.jvm_ops = 4_000;
                config.jvm_loop = 8_000_000;
                config.zip_bytes = 1024 * 1024;
                config.jar_ms = 1200;
            }
            "--width" => config.screen_width = parse_next(&mut args, "--width")?,
            "--height" => config.screen_height = parse_next(&mut args, "--height")?,
            "--grid" => config.grid_size = parse_next(&mut args, "--grid")?,
            "--frames" => config.frames = parse_next(&mut args, "--frames")?,
            "--texture" => config.texture_size = parse_next(&mut args, "--texture")?,
            "--jvm-ops" => config.jvm_ops = parse_next(&mut args, "--jvm-ops")?,
            "--jvm-loop" => config.jvm_loop = parse_next(&mut args, "--jvm-loop")?,
            "--zip-bytes" => config.zip_bytes = parse_next(&mut args, "--zip-bytes")?,
            "--jar" => {
                let value = args.next().with_context(|| "missing value for --jar")?;
                config.extra_jar = Some(PathBuf::from(value));
            }
            "--midlet" => {
                let value = args.next().with_context(|| "missing value for --midlet")?;
                config.midlet = Some(PathBuf::from(value));
            }
            "--jar-ms" => config.jar_ms = parse_next(&mut args, "--jar-ms")?,
            "--suite" => {
                let value = args.next().with_context(|| "missing value for --suite")?;
                suites.extend(parse_suites(&value)?);
            }
            other => bail!("unknown argument: {other}"),
        }
    }
    if suites.is_empty() {
        suites = all_suites();
    }
    suites.dedup();
    Ok(Some((config, suites)))
}

fn parse_suites(value: &str) -> anyhow::Result<Vec<Suite>> {
    let mut suites = Vec::new();
    for part in value.split(',') {
        match part.trim() {
            "all" => return Ok(all_suites()),
            "v3" => suites.push(Suite::V3),
            "m3g" => suites.push(Suite::M3g),
            "lcdui" => suites.push(Suite::LcdUi),
            "load" => suites.push(Suite::Load),
            "jvm" => suites.push(Suite::Jvm),
            "jar" => suites.push(Suite::Jar),
            other => bail!("unknown suite: {other}"),
        }
    }
    Ok(suites)
}

fn all_suites() -> Vec<Suite> {
    vec![Suite::V3, Suite::M3g, Suite::LcdUi, Suite::Load, Suite::Jvm, Suite::Jar]
}

fn suite_name(suite: &Suite) -> &'static str {
    match suite {
        Suite::V3 => "v3",
        Suite::M3g => "m3g",
        Suite::LcdUi => "lcdui",
        Suite::Load => "load",
        Suite::Jvm => "jvm",
        Suite::Jar => "jar",
    }
}

fn parse_next<T>(args: &mut impl Iterator<Item = String>, name: &str) -> anyhow::Result<T>
where
    T: core::str::FromStr,
    T::Err: core::fmt::Display,
{
    let value = args.next().with_context(|| format!("missing value for {name}"))?;
    value.parse().map_err(|err| anyhow!("invalid value for {name}: {value}: {err}"))
}

fn print_help() {
    println!(
        "Usage: cargo run --release --bin engine_bench -- [options]\n\
\n\
Stress every engine on a comparable TSV timeline so drops show up per phase.\n\
\n\
Options:\n\
  --suite LIST   Comma list: all,v3,m3g,lcdui,load,jvm,jar  (default all)\n\
  --width N      Screen width, default 240\n\
  --height N     Screen height, default 320\n\
  --grid N       3D mesh grid size, default 18\n\
  --frames N     Frames per raster/load phase, default 120\n\
  --texture N    Texture size, default 64\n\
  --jvm-ops N    Hashtable/String/Math iterations, default 1500\n\
  --jvm-loop N   Interpreter loop trips, default 2000000\n\
  --zip-bytes N  Synthetic zip payload size, default 262144\n\
  --jar PATH     Also inflate this JAR in the load suite\n\
  --midlet PATH  Portable MIDP bench JAR (default benches/jar/dist/RustJavaBench.jar)\n\
  --jar-ms N     Milliseconds per MIDP bench phase, default 400\n\
  --quick        Smaller workload\n\
  --heavy        Larger workload\n\
  -h, --help     Show this help"
    );
}

fn mix(state: u64, value: u64) -> u64 {
    let value = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    (state ^ value).rotate_left(27).wrapping_mul(0x94d0_49bb_1331_11eb)
}
