# Independent J2ME emulator comparison

Date: 2026-09-08  
Host: Windows, AMD-class desktop, OpenJDK 21 Temurin for host-JVM emulators.  
Workload: `benches/jar/dist/RustJavaBench.jar` — same MIDP 2.0 MIDlet on every emulator, 400 ms timed loop per phase, scores written by the MIDlet itself.

## What was downloaded and run

| emulator | version | source | result |
| --- | --- | --- | --- |
| **RustJava** `engine_bench --suite jar` | this tree, release | native VM | 18/18 scores |
| **RustJava** `rust_java -jar` (window) | this tree, release | native VM | 18/18 scores |
| **FreeJ2ME-Plus** | 1.52 | [TASEmulators/freej2me-plus](https://github.com/TASEmulators/freej2me-plus/releases/tag/1.52) | 18/18 scores |
| **zb3/freej2me** | bootleg-rc5 Windows amd64 | [zb3/freej2me](https://github.com/zb3/freej2me/releases) | 18/18 scores |
| **KEmulator nnmod nnx64** | 2.21.4 | [shinovon/KEmulator](https://github.com/shinovon/KEmulator/releases/tag/v2.21.4) | process starts, MIDlet never produced scores in 70s |
| **MicroEmulator** | 2.0.4 | Maven Central | hung after loading Java 8 classfiles |
| **J2ME-Loader / JL-Mod** | — | Android only | not run on this Windows host |

Raw logs/score files: `scratch/emu-compare/out/`.

## How to read the numbers

Host-JVM emulators execute the MIDlet on **HotSpot JIT**. RustJava **interprets** CLDC bytecode in its own VM. CPU phases (int_add, hashtable, …) are therefore not the same kind of work: they measure HotSpot vs our interpreter, not “who draws phones better”.

Graphics/3D phases still go through each emulator’s LCDUI / M3G / MascotCapsule code, so those ratios are the useful engine comparison.

Every complete run lasts ~8 s of wall time because the JAR is time-boxed (18 × 400 ms). **ops/sec** is the speed metric, not wall time.

## Resources

| emulator | wall s | peak RSS MiB | CPU s | on-disk |
| --- | ---: | ---: | ---: | ---: |
| RustJava engine_bench | 7.9 | 650 | 7.3 | 29 MB exe |
| RustJava window | 7.8 | 639 | 7.3 | 29 MB exe |
| FreeJ2ME-Plus 1.52 | 9.5 | **2324** | 11.5 | 2.2 MB + JRE |
| zb3/freej2me rc5 | 8.4 | **2090** | 9.3 | 12 MB + JRE |
| KEmulator nnx64 (no scores) | 71 (timeout) | 132 | 1.0 | 36 MB + JRE |

RustJava uses about **3.5× less RAM** than FreeJ2ME-Plus/zb3 on this workload. Host-JVM emulators also need a JRE (~200 MB install) at runtime.

## ops/sec (higher is better)

| phase | RustJava | FreeJ2ME-Plus | zb3/freej2me | vs Plus |
| --- | ---: | ---: | ---: | ---: |
| cpu/int_add | 3.73M | 1237M | 1414M | 330× slower |
| cpu/string | 27.5k | 12.4M | 12.4M | 450× slower |
| cpu/hashtable | 27.6k | 32.3M | 36.1M | 1170× slower |
| gfx/fill_rect | 140k | 20.7M | 10.8M | 147× slower |
| gfx/draw_line | 5.7k | 7.3M | 8.6M | **1270× slower** |
| gfx/fill_triangle | 1.43k | 101k | 179k | 71× slower |
| gfx/draw_rgb | 35.0k | 92.1k | 55.0k | **2.6× slower** |
| gfx/draw_image | 51.0k | 220k | 308k | 4.3× slower |
| gfx/draw_string | 35.4k | 900k | 1.59M | 25× slower |
| game/sprite | 49.5k | 1.27M | 293k | 26× slower |
| m3g/transform | 41.0k | 6.62M | 594k | 162× slower |
| v3/affine | 33.4k | 11.1M | 14.6M | 333× slower |

## Verdict

**We do not outperform the current Windows J2ME emulators on this JAR.** FreeJ2ME-Plus and zb3/freej2me run the same MIDlet far faster because guest bytecode is HotSpot-compiled, and their 2D paths (especially `drawLine` / `fillRect`) are much cheaper than our CPU raster.

Where we are closest: **`drawRGB` (2.6×)** and **`drawImage` (4×)**. Where we lose hardest among graphics: **`drawLine`**.

Where we do better: **memory** (~650 MiB vs ~2.1–2.3 GiB) and **not requiring a host JRE** to execute the guest VM.

KEmulator nnmod launched (SWT + jinput) but never executed `startApp` far enough to emit scores in this headless harness; MicroEmulator 2.0.4 stalled on Java 8 classfiles. Those are harness/compat limits, not speed wins.

Re-run:

```powershell
python scratch\emu-compare\run_compare.py
```
