# Build QuinusLang runtime to runtime.o / runtime.obj
# Requires: clang (or gcc on Linux) in PATH
# Output: runtime/runtime.o (Linux/macOS) or runtime/runtime.obj (Windows)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Split-Path -Parent $scriptDir
$runtimeDir = Join-Path $rootDir "runtime"
$outDir = Join-Path $rootDir "dist-runtime"

if (-not (Test-Path $runtimeDir)) {
    Write-Error "Runtime directory not found: $runtimeDir"
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$isWindows = $PSVersionTable.PSVersion.Major -ge 5 -and $env:OS -eq "Windows_NT"
$objName = if ($isWindows) { "runtime.obj" } else { "runtime.o" }
$outPath = Join-Path $outDir $objName

# Try clang first, then gcc
$compiler = $null
foreach ($cc in @("clang", "gcc")) {
    try {
        $null = Get-Command $cc -ErrorAction Stop
        $compiler = $cc
        break
    } catch {
        continue
    }
}

if (-not $compiler) {
    Write-Error "No C compiler found (clang or gcc). Install LLVM or MinGW."
}

Write-Host "Building runtime with $compiler..."

$src = Join-Path $runtimeDir "runtime.c"
$args = @("-c", "-O2", "-o", $outPath, $src)

# Windows: use MSVC-compatible object format for lld-link
if ($isWindows) {
    $args += @("-fno-exceptions", "-fno-rtti")
}

& $compiler @args
if ($LASTEXITCODE -ne 0) {
    Write-Error "Runtime build failed"
}

Write-Host "Built: $outPath"
