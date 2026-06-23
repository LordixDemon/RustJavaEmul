param(
    [ValidateSet("debug", "release")]
    [string] $Profile = "release"
)

$ErrorActionPreference = "Stop"

$Repo = Split-Path -Parent $PSScriptRoot
$TargetDir = Join-Path $Repo "target\wasm32-unknown-unknown"
$OutDir = Join-Path $Repo "web\pkg"

$cargoArgs = @(
    "build",
    "--target", "wasm32-unknown-unknown",
    "--no-default-features",
    "--features", "browser-window"
)

if ($Profile -eq "release") {
    $cargoArgs += "--release"
}

cargo @cargoArgs

$profileDir = if ($Profile -eq "release") { "release" } else { "debug" }
$wasm = Join-Path $TargetDir "$profileDir\rust_java.wasm"
if (!(Test-Path $wasm)) {
    throw "WASM output was not found: $wasm"
}

if (!(Get-Command wasm-bindgen -ErrorAction SilentlyContinue)) {
    throw "wasm-bindgen CLI is not installed. Install matching version: cargo install wasm-bindgen-cli --version 0.2.125"
}

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null
wasm-bindgen $wasm --target web --out-dir $OutDir --out-name rust_java
