# Q++ Benchmark Suite - Run all benchmarks and report mean/stddev
# Usage: .\run.ps1 [-Runs N] [-Release]
# Prerequisites: qpp (target/release/qpp.exe), gcc/clang, rustc, zig

param(
    [int]$Runs = 5,
    [switch]$Release
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

# Resolve compiler paths
$Qpp = if (Test-Path "$RootDir\target\release\qpp.exe") {
    "$RootDir\target\release\qpp.exe"
} elseif (Test-Path "$RootDir\target\debug\qpp.exe") {
    "$RootDir\target\debug\qpp.exe"
} else {
    throw "Q++ compiler not found. Run: cargo build --release"
}

$Benchmarks = @("sum", "fib", "sieve", "mandelbrot", "nbody")
$Languages = @("q", "c", "rs", "zig")

function Get-Mean {
    param([double[]]$Values)
    $sum = ($Values | Measure-Object -Sum).Sum
    return $sum / $Values.Count
}

function Get-StdDev {
    param([double[]]$Values, [double]$Mean)
    $variance = ($Values | ForEach-Object { ($_ - $Mean) * ($_ - $Mean) } | Measure-Object -Sum).Sum / $Values.Count
    return [Math]::Sqrt($variance)
}

function Build-And-Run {
    param(
        [string]$BenchName,
        [string]$Lang,
        [string]$ExePath
    )
    $timings = @()
    for ($i = 0; $i -lt $Runs; $i++) {
        $sw = [System.Diagnostics.Stopwatch]::StartNew()
        $proc = Start-Process -FilePath $ExePath -WindowStyle Hidden -PassThru -Wait
        $sw.Stop()
        if ($proc.ExitCode -ne 0) {
            Write-Warning "$BenchName ($Lang) run $i exited with code $($proc.ExitCode)"
        }
        $timings += $sw.Elapsed.TotalSeconds
    }
    $mean = Get-Mean $timings
    $stddev = Get-StdDev $timings $mean
    return @{ Mean = $mean; StdDev = $stddev }
}

function Build-Qpp {
    param([string]$Path)
    $qPath = Join-Path $Path "main.q"
    if (-not (Test-Path $qPath)) { return $false }
    $buildDir = Join-Path $Path "build"
    if (-not (Test-Path $buildDir)) { New-Item -ItemType Directory -Path $buildDir | Out-Null }
    $args = @("build", $qPath)
    if ($Release) { $args += "--release" }
    & $Qpp $args 2>&1 | Out-Null
    $exe = Join-Path $buildDir "output.exe"
    return (Test-Path $exe)
}

function Build-C {
    param([string]$Path)
    $cPath = Join-Path $Path "main.c"
    if (-not (Test-Path $cPath)) { return $false }
    $buildDir = Join-Path $Path "build"
    if (-not (Test-Path $buildDir)) { New-Item -ItemType Directory -Path $buildDir | Out-Null }
    $exe = Join-Path $buildDir "main_c.exe"
    if (Get-Command gcc -ErrorAction SilentlyContinue) {
        $opt = if ($Release) { "-O2" } else { "-O0" }
        & gcc $opt -o $exe $cPath -lm 2>&1 | Out-Null
    } elseif (Get-Command cl -ErrorAction SilentlyContinue) {
        $opt = if ($Release) { "/O2" } else { "/Od" }
        & cl $opt "/Fe$exe" $cPath 2>&1 | Out-Null
    } else {
        return $false
    }
    return (Test-Path $exe)
}

function Build-Rust {
    param([string]$Path)
    $rsPath = Join-Path $Path "main.rs"
    if (-not (Test-Path $rsPath)) { return $false }
    $buildDir = Join-Path $Path "build"
    if (-not (Test-Path $buildDir)) { New-Item -ItemType Directory -Path $buildDir | Out-Null }
    $exe = Join-Path $buildDir "main_rs.exe"
    $opt = if ($Release) { "-O" } else { "" }
    rustc $opt -o $exe $rsPath 2>&1 | Out-Null
    return (Test-Path $exe)
}

function Build-Zig {
    param([string]$Path)
    $zigPath = Join-Path $Path "main.zig"
    if (-not (Test-Path $zigPath)) { return $false }
    $buildDir = Join-Path $Path "build"
    if (-not (Test-Path $buildDir)) { New-Item -ItemType Directory -Path $buildDir | Out-Null }
    $exe = Join-Path $buildDir "main_zig.exe"
    $opt = if ($Release) { "-OReleaseFast" } else { "-ODebug" }
    Push-Location $Path
    try {
        zig build-exe $opt main.zig 2>&1 | Out-Null
        if (Test-Path "main.exe") {
            Move-Item -Force "main.exe" $exe
        }
    } finally {
        Pop-Location
    }
    return (Test-Path $exe)
}

Write-Host "Q++ Benchmark Suite" -ForegroundColor Cyan
Write-Host "Runs: $Runs | Release: $Release"
Write-Host ""

$results = @{}  # $results["benchname"]["lang"] = @{ Mean, StdDev }

foreach ($bench in $Benchmarks) {
    $benchPath = Join-Path $ScriptDir $bench
    $results[$bench] = @{}
    
    # Q++
    if (Build-Qpp $benchPath) {
        $exe = Join-Path $benchPath "build\output.exe"
        $r = Build-And-Run $bench "q" $exe
        $results[$bench]["q"] = $r
    }
    
    # C
    if (Build-C $benchPath) {
        $exe = Join-Path $benchPath "build\main_c.exe"
        $r = Build-And-Run $bench "c" $exe
        $results[$bench]["c"] = $r
    }
    
    # Rust
    if (Build-Rust $benchPath) {
        $exe = Join-Path $benchPath "build\main_rs.exe"
        $r = Build-And-Run $bench "rs" $exe
        $results[$bench]["rs"] = $r
    }
    
    # Zig
    if (Build-Zig $benchPath) {
        $exe = Join-Path $benchPath "build\main_zig.exe"
        $r = Build-And-Run $bench "zig" $exe
        $results[$bench]["zig"] = $r
    }
}

# Print table
Write-Host "Results (seconds, mean +/- stddev):" -ForegroundColor Cyan
Write-Host ("{0,-12} | {1,-12} | {2,-12} | {3,-12} | {4,-12}" -f "Benchmark", "Q++", "C", "Rust", "Zig")
Write-Host ("{0,-12}-+-{1,-12}-+-{2,-12}-+-{3,-12}-+-{4,-12}" -f "------------", "------------", "------------", "------------", "------------")

foreach ($bench in $Benchmarks) {
    $row = @()
    foreach ($lang in @("q", "c", "rs", "zig")) {
        if ($results[$bench][$lang]) {
            $m = $results[$bench][$lang].Mean
            $s = $results[$bench][$lang].StdDev
            $row += "{0:F4}+/-{1:F4}" -f $m, $s
        } else {
            $row += "-"
        }
    }
    Write-Host ("{0,-12} | {1,-12} | {2,-12} | {3,-12} | {4,-12}" -f $bench, $row[0], $row[1], $row[2], $row[3])
}

Write-Host ""
Write-Host "Done."
