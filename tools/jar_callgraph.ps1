param(
    [Parameter(Mandatory = $true)]
    [string]$Jar,

    [string]$OutDir = "run_logs\jar_callgraph"
)

$ErrorActionPreference = "Stop"

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

$classes = & jar tf $Jar |
    Where-Object { $_ -like "*.class" } |
    ForEach-Object { $_.Substring(0, $_.Length - 6).Replace("/", ".") }

$methodRows = New-Object System.Collections.Generic.List[object]
$callRows = New-Object System.Collections.Generic.List[object]

foreach ($class in $classes) {
    $lines = & javap -classpath $Jar -c -p $class 2>$null
    $currentMethod = ""

    foreach ($line in $lines) {
        $trim = $line.Trim()

        if (
            $trim -notmatch "^\d+:" -and
            $trim -match "^(public|private|protected|static|final|synchronized|native|abstract|strictfp|[A-Za-z_$][\w$./\[\]]+).*\(.*\).*[;]$" -and
            $trim -notmatch "^Compiled from"
        ) {
            $currentMethod = $trim
            $methodRows.Add([pscustomobject]@{
                class = $class
                method = $currentMethod
            })
            continue
        }

        if ($line -match "^\s*(\d+):\s+(invoke\w+|getfield|putfield|getstatic|putstatic|new|anewarray|checkcast|instanceof)\s+#\d+(?:,\s+\d+)?\s+//\s+(.+)$") {
            $callRows.Add([pscustomobject]@{
                source_class = $class
                source_method = $currentMethod
                bci = [int]$Matches[1]
                opcode = $Matches[2]
                target = $Matches[3]
            })
        }
    }
}

$external = $callRows | Where-Object { $_.target -match "(java/|javax/|com/mascotcapsule/|com/nokia/)" }

$methodRows | Export-Csv -NoTypeInformation -Encoding UTF8 -Path "$OutDir\methods.csv"
$callRows | Export-Csv -NoTypeInformation -Encoding UTF8 -Path "$OutDir\callgraph.csv"
$external | Export-Csv -NoTypeInformation -Encoding UTF8 -Path "$OutDir\external_calls.csv"

[pscustomobject]@{
    jar = $Jar
    classes = @($classes).Count
    methods = $methodRows.Count
    call_sites = $callRows.Count
    external_call_sites = @($external).Count
    unique_external_targets = @($external | Sort-Object target -Unique).Count
} | ConvertTo-Json | Set-Content -Encoding UTF8 -Path "$OutDir\summary.json"

Get-Content "$OutDir\summary.json"
