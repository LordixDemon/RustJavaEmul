param(
    [Parameter(Position = 0)]
    [ValidateSet("status", "health", "screenshot", "tap", "down", "up", "keys", "help")]
    [string] $Command = "status",
    [Parameter(Position = 1)]
    [string] $Key = "ok",
    [string] $Out,
    [int] $Hold = 3
)

$ErrorActionPreference = "Stop"
$Repo = Split-Path -Parent $PSScriptRoot
$UrlFile = Join-Path $Repo "target\emu_control.url"
if (!(Test-Path $UrlFile)) {
    throw "Emulator control is not running. Start rust_java first; expected $UrlFile"
}
$Base = (Get-Content -Raw $UrlFile).Trim().TrimEnd("/")
if ([string]::IsNullOrWhiteSpace($Base)) {
    throw "Empty control URL in $UrlFile"
}

function Invoke-Emu([string]$Path, [string]$OutFile) {
    $uri = "$Base$Path"
    if ($OutFile) {
        Invoke-WebRequest -UseBasicParsing -Uri $uri -OutFile $OutFile | Out-Null
        return $OutFile
    }
    $response = Invoke-WebRequest -UseBasicParsing -Uri $uri
    if ($response.Content -is [byte[]]) {
        return [System.Text.Encoding]::UTF8.GetString($response.Content)
    }
    return [string]$response.Content
}

switch ($Command) {
    "help" {
        Invoke-Emu "/help"
        break
    }
    "health" {
        Invoke-Emu "/health"
        break
    }
    "status" {
        Invoke-Emu "/status"
        break
    }
    "keys" {
        Invoke-Emu "/keys"
        break
    }
    "screenshot" {
        if (-not $Out) {
            $dir = Join-Path $Repo "target\screenshots"
            New-Item -ItemType Directory -Force -Path $dir | Out-Null
            $stamp = Get-Date -Format "yyyyMMdd-HHmmss"
            $Out = Join-Path $dir "lcd-ctl-$stamp.png"
        }
        $saved = Invoke-Emu "/screenshot" $Out
        Write-Output $saved
        break
    }
    "tap" {
        Invoke-Emu "/tap?key=$([uri]::EscapeDataString($Key))&hold=$Hold"
        break
    }
    "down" {
        Invoke-Emu "/down?key=$([uri]::EscapeDataString($Key))"
        break
    }
    "up" {
        Invoke-Emu "/up?key=$([uri]::EscapeDataString($Key))"
        break
    }
}
