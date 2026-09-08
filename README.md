# RustJava

RustJava is a Rust JVM and Java ME runtime for running MIDP JARs on the desktop and in the browser.

```powershell
rust_java --help
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

## Quick Start

Run a local JAR in the desktop window:

```powershell
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

Override the virtual phone screen size when needed:

```powershell
$env:RUSTJAVA_SCREEN = "240x320"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

## Engine benchmarks

Max-stress every CPU engine on one TSV timeline (MascotCapsule v3, M3G, LCDUI 2D, classfile/zip, JVM, portable MIDP JAR):

```powershell
cargo run --release --bin engine_bench -- --quick
cargo run --release --bin engine_bench -- --suite jar --quick
cargo run --release --bin engine_bench -- --suite m3g,lcdui --heavy
```

`--suite jar` runs `benches/jar/dist/RustJavaBench.jar` headless. That same JAR is the comparable workload on KEmulator, FreeJ2ME+, or J2ME-Loader: scores are ops/sec per phase on stdout (`RUSTJAVA_BENCH` lines). Rebuild with `.\benches\jar\build.ps1`.

`v3_bench` is the older MascotCapsule-only binary. Compare `ops_per_sec` per `suite`/`phase` row to spot a drop.

## Browser (WASM)

Toolchain needs the `wasm32-unknown-unknown` target (see `rust-toolchain.toml`) and matching wasm-bindgen:

```powershell
cargo install wasm-bindgen-cli --version 0.2.125
.\tools\build_web.ps1
python tools\web_server.py
```

Open `http://localhost:8080/` and choose a JAR. Add `?diag=1` for renderer/timing logs.

3D / WebGPU only works in a **secure context**. For a phone on the LAN:

```powershell
.\tools\create_https_cert.ps1
python tools\web_server.py --cert tools\certs\rustjava-dev.crt --key tools\certs\rustjava-dev.key
```

Linux/macOS CI uses `tools/build_web.sh`.

## GPU

3D is drawn on the GPU through wgpu. The host picks a real GPU backend and only falls back to GL/WebGL when Vulkan/Metal/DX12/WebGPU cannot present:

- Desktop / phones / handhelds: Vulkan, then Metal, then D3D12, then GLES/EGL (needed on many ARM boards and Wayland).
- Browser: WebGPU, then WebGL. `canvas2d` is only used when there is no GPU (or the page is not a secure context).
- MascotCapsule v3 scenes that the GPU path accepts are submitted as GPU triangles. They are not software-rasterized on the CPU.
- JSR-184 M3G submits projected triangles, depth, and up to two texture units to wgpu when a GPU presenter is active. Vertex transform, lighting, skinning, and clip stay on the CPU. `render(World)` with color-clear replaces the GPU frame; later `render(Node)` calls merge into it. HUD 2D is composited after 3D through the screen alpha channel.

Force a backend with `WGPU_BACKEND` (wgpu) or `RUSTJAVA_GPU` (same names: `vulkan`, `dx12`, `metal`, `gl`, `webgpu`). `WGPU_BACKEND` wins if both are set.

```powershell
$env:WGPU_BACKEND = "vulkan"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

The window title and `rendererInfo()` show the adapter that was actually selected (`backend=Vulkan`, `BrowserWebGpu`, `Gl`, …).

## Limitations

- There is no real audio. Tone/`Player` APIs mostly change state.
- HTTP on WASM returns `Unsupported`. Desktop HTTP is raw HTTP/1.0 without TLS.
- Many vendor/Java SE classes are registered as no-op stubs so JARs *load*. A game can still throw a Java exception as soon as it needs real behavior.
- Static “100% coverage” in `docs/compat_matrix.md` counts those stubs. It is not a playability guarantee.
- Browser GPU / MascotCapsule v3 needs HTTPS or localhost. WASM uses WebGPU when the browser has it, otherwise WebGL.
- JSR-184 M3G GPU raster needs an active GPU presenter. Offscreen `Image` targets stay on the CPU. Lighting and skinning stay on the CPU. The GPU path samples both texture units (REPLACE/DECAL/ADD/BLEND/MODULATE), applies LINEAR/EXP fog per pixel, and overlays MIDP HUD after 3D.

## Java ME corpus from Spaces.im

Collect a device-diverse catalog and download at least 500 verified Java ME
games into `apk\spaces-java`:

```powershell
python tools\spaces_java_parser.py corpus --target 500 --candidates 800
```

The command is resumable.  It writes `catalog.json`, `catalog.csv`,
`catalog.md`, and `download-report.json`; games are grouped by phone family.
To resume an interrupted run, execute the same command again.

The same parser can exhaustively collect the Java programs section and the
complete Java games section.  These commands follow every pagination link;
downloads are resumable and keep verified JARs under `apk\spaces-java`:

```powershell
python tools\spaces_java_parser.py --proxy-file IPV4.txt catalog --section programs --all-pages --output apk\spaces-java\programs-catalog.json --delay-ms 0
python tools\spaces_java_parser.py --proxy-file IPV4.txt download apk\spaces-java\programs-catalog.json --output apk\spaces-java\programs --workers 32 --delay-ms 0

python tools\spaces_java_parser.py --proxy-file IPV4.txt catalog --section games --all-pages --output apk\spaces-java\games-catalog.json --delay-ms 0
python tools\spaces_java_parser.py --proxy-file IPV4.txt download apk\spaces-java\games-catalog.json --output apk\spaces-java --workers 32 --delay-ms 0
```

The parser rotates proxies from `IPV4.txt` between requests and retries. The
file may use `host:port:user:password` or a full `http://`/`https://` proxy
URL per line. Already verified JARs are skipped when a download is resumed.

The exhaustive catalog commands use the site's current pagination and stop at
the last page.  If the source or connection interrupts a download, rerun the
same `download` command; its report skips already verified files.

## Controls

The host picks a device profile from the JAR (filename, MIDlet manifest, and referenced vendor classes such as `com.nokia` / `com.mascotcapsule` / `com.siemens`). Override with `--device Nokia` or `RUSTJAVA_DEVICE`. The window title shows `Game [Profile]`.

PC keys always mean the same thing; the MIDP key codes they emit follow the phone:

- Arrow keys or `WASD`: d-pad
- `Enter` or `Space`: fire/select
- `Q` / `F1`: left softkey (Siemens **A**)
- `E` / `F2`: right softkey (Siemens **B**)
- `0`-`9`: phone keypad
- Mouse/touch: bottom on-screen control panel
- `F12`: save the LCD framebuffer to `target/screenshots/`
- `Esc`: close the window

Nokia / Sony Ericsson / Samsung / LG use the usual MIDP codes (`UP=-1` … `FIRE=-5`, softkeys `-6`/`-7`). Siemens and Motorola games get that vendor’s native codes so `Canvas.getGameAction` matches the handset.

The desktop window exposes a localhost control port (default `http://127.0.0.1:17420`, URL also written to `target/emu_control.url`). It sends MIDP keys and returns the LCD PNG; it does not capture the desktop or move the mouse.

```powershell
.\tools\emu_ctl.ps1 status
.\tools\emu_ctl.ps1 tap ok
.\tools\emu_ctl.ps1 screenshot
```

Set `RUSTJAVA_CONTROL=0` to disable. `RUSTJAVA_SCREENSHOT_EVERY=2` also dumps the LCD every N seconds.

## Project Map

- `src/`: desktop runner, MIDP window, file system, input, and profiling glue.
- `src/gpu/`: wgpu backend selection and shared MascotCapsule v3 / JSR-184 M3G GPU presenters (desktop + WASM).
- `src/runtime/controls.rs`: desktop control panel drawing and MIDP keyboard mapping.
- `jvm/`: core JVM interfaces and shared object model.
- `jvm_rust/`: bytecode interpreter, stack frames, profiling counters, and runtime class instances.
- `java_runtime/`: Java standard library, MIDP APIs, Nokia APIs, and MascotCapsule classes.
- `java_runtime/src/classes/com/mascotcapsule/micro3d/v3.rs`: MascotCapsule v3 public API and scene submission.
- `java_runtime/src/classes/com/mascotcapsule/micro3d/v3/binary.rs`: shared little-endian and bit-packed resource reader for MTRA/MBAC assets.
- `java_runtime/src/classes/com/mascotcapsule/micro3d/v3/math.rs`: fixed-point vector and affine matrix helpers.
- `java_runtime/src/classes/com/mascotcapsule/micro3d/v3/texture.rs`: indexed BMP texture decoding and model texture padding.
- `classfile/`: Java class file parser.
- `java_class_proto/`: class/method/field prototypes used by runtime classes.
- `java_constants/`: JVM constants shared by crates.
- `test_utils/`, `tests/`, `test_data/`: JVM and runtime tests.
- `apk/`: local game files used for emulator testing.

## Development Commands

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release
.\tools\build_web.ps1
```

Use tracing for runtime diagnostics (`RUST_LOG` is a tracing-subscriber filter, default `warn`):

```powershell
$env:RUST_LOG = "java_runtime::classes::com::mascotcapsule::micro3d::v3=debug"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

Use profiling for CPU-time work (`RUSTJAVA_TIMING`; `RUSTJAVA_PROFILE=1` still enables timing for compatibility):

```powershell
$env:RUSTJAVA_TIMING = "1"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

Force a vendor profile with `RUSTJAVA_DEVICE` (or `--device Nokia`). Do not set `RUSTJAVA_PROFILE=1` if you meant a device name.

| Variable | Role |
|---|---|
| `RUSTJAVA_DEVICE` | Vendor profile (`Nokia`, `SonyEricsson`, …) |
| `RUSTJAVA_TIMING` | CPU profiler (`1` / `true` / `on`) |
| `RUSTJAVA_PROFILE` | Legacy: vendor name, or `1`/`true` to enable timing |
| `RUSTJAVA_SCREEN` | `WIDTHxHEIGHT` (or `--screen`) |
| `RUSTJAVA_GPU` / `WGPU_BACKEND` | wgpu backends |
| `RUSTJAVA_CONTROL` | `0`/`false`/`off`/`no` disables control HTTP |
| `RUSTJAVA_CONTROL_PORT` | Default `17420` |
| `RUSTJAVA_WINDOW_FPS` / `RUSTJAVA_GAME_FPS` | Presenter cap / MIDlet FPS |
| `RUSTJAVA_YIELD_INTERVAL` | Yield every N ticks (default 64) |
| `RUSTJAVA_DIAG` / `RUSTJAVA_INPUT_DIAG` | Diagnostics |


## Rendering Notes

MascotCapsule v3 model textures are 8-bit indexed BMP images. Non-power-of-two textures are padded to the next power of two before UV sampling, matching old phone implementations. Palette index `0` is used as the transparent background for color-keyed and detected decal-like geometry.

Keep renderer changes data-driven:

- Preserve Java game assets and bytecode behavior.
- Prefer compatibility fixes in runtime APIs, texture decoding, projection, sorting, and rasterization.
- Add diagnostics around timing, queue sizes, material flags, UV regions, and frame presentation before optimizing.
- Verify every rendering change with `3d_treasuretowers.jar` and at least one non-MascotCapsule MIDP game.
- Keep 3D on the GPU path (`src/gpu/`). Do not add new CPU raster fast-paths for MascotCapsule v3 or JSR-184 M3G World renders when a scene can be published to wgpu.
