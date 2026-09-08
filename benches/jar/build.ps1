$ErrorActionPreference = "Stop"
$root = $PSScriptRoot
$stubOut = Join-Path $root "build\stubs"
$classOut = Join-Path $root "build\classes"
$dist = Join-Path $root "dist"
$jar = Join-Path $dist "RustJavaBench.jar"

if (Test-Path (Join-Path $root "build")) {
    Remove-Item -Recurse -Force (Join-Path $root "build")
}
New-Item -ItemType Directory -Force -Path $stubOut, $classOut, $dist | Out-Null

$stubFiles = @(Get-ChildItem -Recurse (Join-Path $root "stubs") -Filter *.java | ForEach-Object { $_.FullName })
$srcFiles = @(Get-ChildItem -Recurse (Join-Path $root "src") -Filter *.java | ForEach-Object { $_.FullName })

Write-Host "javac stubs ($($stubFiles.Count) files)"
& javac --release 8 -Xlint:-options -encoding UTF-8 -d $stubOut @stubFiles
if ($LASTEXITCODE -ne 0) { throw "stub compile failed" }

Write-Host "javac sources ($($srcFiles.Count) files)"
& javac --release 8 -Xlint:-options -encoding UTF-8 -cp $stubOut -d $classOut @srcFiles
if ($LASTEXITCODE -ne 0) { throw "source compile failed" }

Write-Host "jar $jar"
Push-Location $classOut
try {
    & jar cfm $jar (Join-Path $root "MANIFEST.MF") org
    if ($LASTEXITCODE -ne 0) { throw "jar failed" }
} finally {
    Pop-Location
}

Write-Host "built $jar ($((Get-Item $jar).Length) bytes)"
