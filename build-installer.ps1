# Build QuinusLang and create the installer
# Requires: Inno Setup 6 (https://jrsoftware.org/isdl.php)
# Usage: .\build-installer.ps1

$ErrorActionPreference = "Stop"

Write-Host "Building QuinusLang..."
& "$PSScriptRoot\build.ps1"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
}

if (-not (Test-Path "quinus.exe")) {
    Write-Error "quinus.exe not found after build"
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
