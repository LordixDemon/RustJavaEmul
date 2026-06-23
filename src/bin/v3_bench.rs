use std::{env, time::Instant};

use anyhow::{Context, anyhow, bail};
use java_runtime::classes::com::mascotcapsule::micro3d::v3::benchmark::{V3Benchmark, V3BenchmarkConfig, V3BenchmarkRun};

fn main() -> anyhow::Result<()> {
    let Some(config) = parse_args()? else {
        print_help();
        return Ok(());
    };

    let benchmark = V3Benchmark::new(config);
    let config = benchmark.config();
    let prepared = benchmark.prepare_frame(0);

    println!(
        "v3_bench\tversion={}\tos={}\tarch={}\twidth={}\theight={}\tgrid={}\tframes={}\ttexture={}\tvertices={}\tsource_quads={}\tprepared_triangles={}\tprepared_checksum={:016x}",
        env!("CARGO_PKG_VERSION"),
        env::consts::OS,
        env::consts::ARCH,
        config.screen_width,
        config.screen_height,
        config.grid_size,
        config.frames,
        config.texture_size,
        benchmark.vertex_count(),
        benchmark.source_quads(),
        prepared.triangle_count(),
        prepared.checksum(),
    );
    println!("phase\tframes\tvertices\tquads\ttriangles\tchanged_triangles\tchecksum\ttotal_ms\tms_per_frame\ttriangles_per_sec");

    let (geometry_time, geometry) = measure(|| benchmark.run_geometry());
    print_result("geometry", geometry_time, geometry);

    let (raster_time, raster) = measure(|| benchmark.run_raster(&prepared));
    print_result("raster", raster_time, raster);

    let (full_time, full) = measure(|| benchmark.run_full_frame());
    print_result("full_frame", full_time, full);

    Ok(())
}

fn measure(run: impl FnOnce() -> V3BenchmarkRun) -> (std::time::Duration, V3BenchmarkRun) {
    let start = Instant::now();
    let result = run();
    (start.elapsed(), result)
}

fn print_result(phase: &str, duration: std::time::Duration, run: V3BenchmarkRun) {
    let seconds = duration.as_secs_f64().max(f64::MIN_POSITIVE);
    let total_ms = seconds * 1000.0;
    let ms_per_frame = total_ms / run.frames.max(1) as f64;
    let triangles_per_sec = run.triangles as f64 / seconds;
    println!(
        "{}\t{}\t{}\t{}\t{}\t{}\t{:016x}\t{:.3}\t{:.4}\t{:.0}",
        phase,
        run.frames,
        run.vertices_per_frame,
        run.source_quads,
        run.triangles,
        run.changed_triangles,
        run.checksum,
        total_ms,
        ms_per_frame,
        triangles_per_sec,
    );
}

fn parse_args() -> anyhow::Result<Option<V3BenchmarkConfig>> {
    let mut config = V3BenchmarkConfig::default();
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "--quick" => {
                config.frames = 30;
                config.grid_size = 12;
            }
            "--heavy" => {
                config.frames = 180;
                config.grid_size = 28;
            }
            "--width" => config.screen_width = parse_next(&mut args, "--width")?,
            "--height" => config.screen_height = parse_next(&mut args, "--height")?,
            "--grid" => config.grid_size = parse_next(&mut args, "--grid")?,
            "--frames" => config.frames = parse_next(&mut args, "--frames")?,
            "--texture" => config.texture_size = parse_next(&mut args, "--texture")?,
            other => bail!("unknown argument: {other}"),
        }
    }
    Ok(Some(config))
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
        "Usage: cargo run --release --bin v3_bench -- [options]\n\
\n\
Options:\n\
  --width N      Screen width, default 240\n\
  --height N     Screen height, default 320\n\
  --grid N       Synthetic mesh grid size, default 18\n\
  --frames N     Frames per phase, default 120\n\
  --texture N    Indexed texture size, default 64\n\
  --quick        Smaller workload: frames=30 grid=12\n\
  --heavy        Larger workload: frames=180 grid=28\n\
  -h, --help     Show this help"
    );
}
