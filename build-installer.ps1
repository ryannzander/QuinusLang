# Build QuinusLang and create the installer
# Requires: Inno Setup 6 (https://jrsoftware.org/isdl.php)
# Usage: .\build-installer.ps1
#
# Packages the Rust-built quinus (full CLI: build, run, init, --version)
# NOT the Q-built compiler (minimal driver)

$ErrorActionPreference = "Stop"

Write-Host "Building QuinusLang (Rust)..."
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Error "Rust build failed"
}

# Use Rust quinus for installer - has full CLI (build, run, init, etc.)
Copy-Item "target\release\quinus.exe" "quinus.exe" -Force
if (-not (Test-Path "quinus.exe")) {
    Write-Error "quinus.exe not found"
}

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
