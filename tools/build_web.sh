#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROFILE="${1:-release}"
TARGET_DIR="$ROOT/target/wasm32-unknown-unknown"
OUT_DIR="$ROOT/web/pkg"

set_web_build_id() {
  local stamp="$1"
  python3 - "$ROOT" "$stamp" <<'PY'
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
stamp = sys.argv[2]
app_js = root / "web" / "app.js"
index = root / "web" / "index.html"
app_js.write_text(re.sub(r'const BUILD_ID = "[^"]+"', f'const BUILD_ID = "{stamp}"', app_js.read_text(encoding="utf-8")), encoding="utf-8")
index.write_text(re.sub(r'app\.js\?v=[^"]+', f"app.js?v={stamp}", index.read_text(encoding="utf-8")), encoding="utf-8")
PY
}

cargo_args=(
  build
  --target wasm32-unknown-unknown
  --no-default-features
  --features browser-window
)

if [[ "$PROFILE" == "release" ]]; then
  cargo_args+=(--release)
fi

(cd "$ROOT" && cargo "${cargo_args[@]}")

profile_dir="release"
if [[ "$PROFILE" != "release" ]]; then
  profile_dir="debug"
fi

wasm="$TARGET_DIR/$profile_dir/rust_java.wasm"
if [[ ! -f "$wasm" ]]; then
  echo "WASM output was not found: $wasm" >&2
  exit 1
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "wasm-bindgen CLI is not installed. Install matching version: cargo install wasm-bindgen-cli --version 0.2.125" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"
wasm-bindgen "$wasm" --target web --out-dir "$OUT_DIR" --out-name rust_java

stamp="$(date +%Y%m%d-%H%M%S)"
set_web_build_id "$stamp"
echo "Web build id: $stamp"
