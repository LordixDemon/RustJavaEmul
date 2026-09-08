param(
    [ValidateSet("debug", "release")]
    [string] $Profile = "release"
)

$ErrorActionPreference = "Stop"

$Repo = Split-Path -Parent $PSScriptRoot
$TargetDir = Join-Path $Repo "target\wasm32-unknown-unknown"
$OutDir = Join-Path $Repo "web\pkg"

function Set-WebBuildId([string]$Stamp) {
    $appJsPath = Join-Path $Repo "web\app.js"
    $indexPath = Join-Path $Repo "web\index.html"
    $appJs = [System.IO.File]::ReadAllText($appJsPath)
    $appJs = [regex]::Replace($appJs, 'const BUILD_ID = "[^"]+"', "const BUILD_ID = `"$Stamp`"")
    [System.IO.File]::WriteAllText($appJsPath, $appJs)
    $index = [System.IO.File]::ReadAllText($indexPath)
    $index = [regex]::Replace($index, 'app\.js\?v=[^"]+', "app.js?v=$Stamp")
    [System.IO.File]::WriteAllText($indexPath, $index)
}

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
if ($LASTEXITCODE -ne 0) {
    throw "cargo build failed with exit code $LASTEXITCODE"
}

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
if ($LASTEXITCODE -ne 0) {
    throw "wasm-bindgen failed with exit code $LASTEXITCODE"
}

$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
Set-WebBuildId $stamp
Write-Host "Web build id: $stamp"
