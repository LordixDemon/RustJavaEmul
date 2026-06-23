param(
    [string]$Ip = "",
    [string]$CertDir = "tools\certs",
    [int]$Days = 30
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($Ip)) {
    $Ip = Get-NetIPAddress -AddressFamily IPv4 |
        Where-Object { $_.IPAddress -like "192.168.*" } |
        Select-Object -First 1 -ExpandProperty IPAddress
}

if ([string]::IsNullOrWhiteSpace($Ip)) {
    throw "LAN IP was not found. Pass -Ip explicitly."
}

$certPath = Join-Path $CertDir "rustjava-dev.crt"
$keyPath = Join-Path $CertDir "rustjava-dev.key"
$configPath = Join-Path $CertDir "rustjava-dev.cnf"

New-Item -ItemType Directory -Force -Path $CertDir | Out-Null

$config = @"
[req]
distinguished_name = req_distinguished_name
x509_extensions = v3_req
prompt = no

[req_distinguished_name]
CN = RustJava Dev HTTPS

[v3_req]
subjectAltName = @alt_names
keyUsage = digitalSignature, keyEncipherment
extendedKeyUsage = serverAuth

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
IP.2 = $Ip
"@

Set-Content -Path $configPath -Value $config -Encoding ASCII

openssl req `
    -x509 `
    -newkey rsa:2048 `
    -sha256 `
    -days $Days `
    -nodes `
    -keyout $keyPath `
    -out $certPath `
    -config $configPath

Write-Host "CERT=$certPath"
Write-Host "KEY=$keyPath"
Write-Host "IP=$Ip"
