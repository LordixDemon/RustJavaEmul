# RustJavaBench

Portable MIDP 2.0 / CLDC 1.1 JAR for comparing RustJava with other emulators (KEmulator, FreeJ2ME+, J2ME-Loader, WTK).

The same `dist/RustJavaBench.jar` is the workload. Scores are ops/sec on a timed loop; higher is better. M3G and MascotCapsule tests load by name and are skipped if the emulator does not have those APIs.

## Run

RustJava (headless, TSV with the rest of `engine_bench`):

```powershell
cargo run --release --bin engine_bench -- --suite jar --quick
```

Any other emulator: open `benches/jar/dist/RustJavaBench.jar`. Tests start in `startApp` and print `RUSTJAVA_BENCH` lines to stdout.

Rebuild the JAR:

```powershell
.\benches\jar\build.ps1
```

Optional duration override (milliseconds per phase, 50–10000):

- system property `rustjava.bench.ms`
- manifest `Bench-Ms` (default 400)
