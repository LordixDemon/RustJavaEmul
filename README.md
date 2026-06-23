# RustJava

RustJava is a Rust JVM and Java ME runtime focused on running MIDP games with a native desktop test window.

## Quick Start

Run the current MascotCapsule test game:

```powershell
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

Run another bundled game:

```powershell
cargo run --release -- -jar "apk\Soul_Of_Darkness-SE(240x320)-spaces.im.jar"
```

Override the virtual phone screen size when needed:

```powershell
$env:RUSTJAVA_SCREEN = "240x320"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

## Controls

- Arrow keys or `WASD`: d-pad
- `Enter` or `Space`: fire/select
- `0`-`9`: phone keypad
- Mouse/touch: bottom on-screen control panel
- `Esc`: close the window

## Project Map

- `src/`: desktop runner, MIDP window, file system, input, and profiling glue.
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
cargo fmt --check
cargo check -p java_runtime
cargo test
cargo build --release
```

Use tracing for runtime diagnostics:

```powershell
$env:RUST_LOG = "java_runtime::classes::com::mascotcapsule::micro3d::v3=debug"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

Use profiling for CPU-time work:

```powershell
$env:RUSTJAVA_PROFILE = "1"
cargo run --release -- -jar "apk\3d_treasuretowers.jar"
```

## Rendering Notes

MascotCapsule v3 model textures are 8-bit indexed BMP images. Non-power-of-two textures are padded to the next power of two before UV sampling, matching old phone implementations. Palette index `0` is used as the transparent background for color-keyed and detected decal-like geometry.

Keep renderer changes data-driven:

- Preserve Java game assets and bytecode behavior.
- Prefer compatibility fixes in runtime APIs, texture decoding, projection, sorting, and rasterization.
- Add diagnostics around timing, queue sizes, material flags, UV regions, and frame presentation before optimizing.
- Verify every rendering change with `3d_treasuretowers.jar` and at least one non-MascotCapsule MIDP game.
