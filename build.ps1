# QuinusLang build script
# First build: requires Rust (cargo build --release)
# Subsequent builds: Q compiler + C compiler only
#
# Usage: .\build.ps1

$ErrorActionPreference = "Stop"

# Find quinus executable (Rust-built or pre-built)
$quinus = $null
if (Test-Path "target\release\quinus.exe") {
    $quinus = "target\release\quinus.exe"
} elseif (Test-Path "quinus.exe") {
    $quinus = "quinus.exe"
} elseif (Test-Path "target\debug\quinus.exe") {
    $quinus = "target\debug\quinus.exe"
}

# Seed build: use Rust if no quinus
if (-not $quinus) {
    Write-Host "No quinus.exe found. Building with Rust (first-time bootstrap)..."
    cargo build --release
    $quinus = "target\release\quinus.exe"
    if (-not (Test-Path $quinus)) {
        Write-Error "Rust build failed"
    }
}

Write-Host "Building compiler with $quinus..."
& $quinus build compiler/main.q
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
}

# Output is in compiler/build/output.exe (the Q-built compiler)
$qCompiler = "compiler\build\output.exe"
if (Test-Path $qCompiler) {
    Copy-Item $qCompiler "quinus.exe" -Force
    Write-Host "quinus.exe ready (from Q compiler)"
} else {
    Write-Host "Build produced compiler\build\output.exe"
}
