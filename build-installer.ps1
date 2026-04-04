# Build Q++ and create the installer
# Requires: Inno Setup 6 (https://jrsoftware.org/isdl.php)
# Usage: .\build-installer.ps1
#
# Packages the Rust-built qpp (full CLI: build, run, init, --version)
# NOT the Q-built compiler (minimal driver)

$ErrorActionPreference = "Stop"

Write-Host "Building Q++ (Rust)..."
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Rust build failed"
}

# Use Rust qpp for installer - has full CLI (build, run, init, etc.)
Copy-Item "target\release\qpp.exe" "qpp.exe" -Force
if (-not (Test-Path "qpp.exe")) {
    Write-Error "qpp.exe not found"
}

# Build runtime and copy lld for bundling
Write-Host "Building runtime..."
& "$PSScriptRoot\scripts\build-runtime.ps1"
if (Test-Path "dist-runtime\runtime.obj") {
    Copy-Item "dist-runtime\runtime.obj" "runtime.obj" -Force
}
$llvmPath = $env:LLVM_SYS_181_PREFIX
if (-not $llvmPath) { $llvmPath = $env:LLVM_SYS_170_PREFIX }
if (-not $llvmPath) { $llvmPath = "C:\Program Files\LLVM" }
$lldLink = Join-Path $llvmPath "bin\lld-link.exe"
if (Test-Path $lldLink) {
    Copy-Item $lldLink "lld-link.exe" -Force
} else {
    Write-Warning "lld-link.exe not found - installer will work but users need LLVM/lld to link"
}
Get-ChildItem (Join-Path $llvmPath "bin\*.dll") -ErrorAction SilentlyContinue | Copy-Item -Destination "." -Force

$iscc = "C:\Program Files (x86)\Inno Setup 6\ISCC.exe"
if (-not (Test-Path $iscc)) {
    $iscc = "C:\Program Files\Inno Setup 6\ISCC.exe"
}
if (-not (Test-Path $iscc)) {
    Write-Error "Inno Setup not found. Install from https://jrsoftware.org/isdl.php"
}

Write-Host "Creating installer..."
& $iscc "installer.iss"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Installer build failed"
}

$setup = Get-ChildItem "installer_output\*.exe" | Select-Object -First 1
if ($setup) {
    Write-Host "Installer created: $($setup.FullName)"
}
