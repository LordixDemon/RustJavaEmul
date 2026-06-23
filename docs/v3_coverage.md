# Mascot Capsule V3 Coverage Audit

Date: 2026-06-22

Reference used: `target/MascotME/src/com/mascotcapsule/micro3d/v3`.

## API Registration

Public API name coverage against the local MascotME reference is 100% for the runtime V3 classes currently exposed by RustJava.

| Class | Reference public names | Rust registered names | Missing |
| --- | ---: | ---: | --- |
| ActionTable | 6 | 6 | none |
| AffineTrans | 18 | 20 | none |
| Effect3D | 23 | 24 | none |
| Figure | 9 | 9 | none |
| FigureLayout | 15 | 15 | none |
| Graphics3D | 9 | 11 | none |
| Light | 13 | 13 | none |
| Texture | 2 | 2 | none |
| Util3D | 3 | 3 | none |
| Vector3D | 11 | 11 | none |

Extra Rust names are compatibility aliases or helpers: `AffineTrans.rotate`, `AffineTrans.scale`, `Graphics3D.getInstance`, and class initializer entries.

## Remaining Risk

This is API registration coverage, not full emulation coverage.

Known unsupported resource branches still exist:

- `mbac.rs`: unsupported MBAC version, bone format, vertex format, normal format.
- `mtra.rs`: unsupported MTRA versions.

Test coverage is not complete yet. Current V3 tests are benchmark/smoke level; there is not yet a golden corpus that proves all V3 resource variants and rendering paths against reference output.
