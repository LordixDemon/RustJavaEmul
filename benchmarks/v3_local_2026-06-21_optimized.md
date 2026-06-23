# V3 Benchmark - After Raster Optimization

Date: 2026-06-21

Baseline file: `benchmarks/v3_local_2026-06-21.md`

## Environment

- CPU: AMD Ryzen 9 7950X 16-Core Processor
- OS: Microsoft Windows 11 Pro for Workstations, 10.0.26200, x86_64
- Rust: rustc 1.88.0
- Build: `cargo build --release --bin v3_bench`

## Optimizations In This Run

- Incremental edge walking in `rasterize_triangle` and `rasterize_triangle_into_pixels`.
- Incremental UV numerator walking for textured triangles.
- Fast opaque overwrite path for `blend_mode == 0` and alpha `0xff`.
- Direct target pixel indexing after validating the target buffer size.

## Quick

Command:

```powershell
.\target\release\v3_bench.exe --quick
```

Config: width=240 height=320 grid=12 frames=30 texture=64 vertices=169 source_quads=144 prepared_triangles=384 prepared_checksum=1f2a7982e5bdaa34

| phase | frames | vertices | quads | triangles | changed_triangles | checksum | total_ms | ms_per_frame | triangles_per_sec |
| --- | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| geometry | 30 | 169 | 144 | 11520 | 0 | d9d563cd7cb8e633 | 1.235 | 0.0412 | 9324915 |
| raster | 30 | 169 | 144 | 11520 | 10800 | ca36f70f1978c25b | 2.868 | 0.0956 | 4017017 |
| full_frame | 30 | 169 | 144 | 11520 | 10896 | 5c30546f9478f453 | 4.614 | 0.1538 | 2497020 |

## Default

Command:

```powershell
.\target\release\v3_bench.exe --frames 120 --grid 18 --width 240 --height 320 --texture 64
```

Config: width=240 height=320 grid=18 frames=120 texture=64 vertices=361 source_quads=324 prepared_triangles=864 prepared_checksum=3d50a84c2fde312d

| phase | frames | vertices | quads | triangles | changed_triangles | checksum | total_ms | ms_per_frame | triangles_per_sec |
| --- | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| geometry | 120 | 361 | 324 | 103680 | 0 | e9309ca2ee57a3f3 | 3.631 | 0.0303 | 28554904 |
| raster | 120 | 361 | 324 | 103680 | 96960 | 60de1df6a5800291 | 27.923 | 0.2327 | 3713015 |
| full_frame | 120 | 361 | 324 | 103680 | 97463 | afab10a10cb74844 | 42.796 | 0.3566 | 2422656 |

## Heavy

Command:

```powershell
.\target\release\v3_bench.exe --heavy
```

Config: width=240 height=320 grid=28 frames=180 texture=64 vertices=841 source_quads=784 prepared_triangles=2092 prepared_checksum=a6808bac03ba9b9a

| phase | frames | vertices | quads | triangles | changed_triangles | checksum | total_ms | ms_per_frame | triangles_per_sec |
| --- | ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| geometry | 180 | 841 | 784 | 376560 | 0 | 2747bfea6f24ce0d | 24.379 | 0.1354 | 15446271 |
| raster | 180 | 841 | 784 | 376560 | 350820 | cf54b20a0b2e0321 | 113.562 | 0.6309 | 3315907 |
| full_frame | 180 | 841 | 784 | 376560 | 353098 | 551cd8c67bf08a4a | 156.333 | 0.8685 | 2408708 |

## Single-Run Comparison

| profile | phase | baseline_ms_per_frame | optimized_ms_per_frame | change |
| --- | --- | ---: | ---: | ---: |
| quick | raster | 0.1222 | 0.0956 | -21.8% |
| quick | full_frame | 0.1702 | 0.1538 | -9.6% |
| default | raster | 0.2437 | 0.2327 | -4.5% |
| default | full_frame | 0.3794 | 0.3566 | -6.0% |
| heavy | raster | 0.7153 | 0.6309 | -11.8% |
| heavy | full_frame | 0.9392 | 0.8685 | -7.5% |

All frame checksums match the baseline for the same profile.
