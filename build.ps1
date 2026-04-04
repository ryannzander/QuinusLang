# Q++ build script
# First build: requires Rust (cargo build --release)
# Subsequent builds: Q compiler + C compiler only
#
# Usage: .\build.ps1

$ErrorActionPreference = "Stop"

# Find qpp executable (Rust-built or pre-built)
$qpp = $null
if (Test-Path "target\release\qpp.exe") {
    $qpp = "target\release\qpp.exe"
} elseif (Test-Path "qpp.exe") {
    $qpp = "qpp.exe"
} elseif (Test-Path "target\debug\qpp.exe") {
    $qpp = "target\debug\qpp.exe"
}

# Seed build: use Rust if no qpp
if (-not $qpp) {
    Write-Host "No qpp.exe found. Building with Rust (first-time bootstrap)..."
    cargo build --release
    $qpp = "target\release\qpp.exe"
    if (-not (Test-Path $qpp)) {
        Write-Error "Rust build failed"
    }
}

Write-Host "Building compiler with $qpp..."
& $qpp build compiler/main.q
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
}

# Output is in compiler/build/output.exe (the Q-built compiler)
$qCompiler = "compiler\build\output.exe"
if (Test-Path $qCompiler) {
    Copy-Item $qCompiler "qpp.exe" -Force
    Write-Host "qpp.exe ready (from Q compiler)"
} else {
    Write-Host "Build produced compiler\build\output.exe"
}
