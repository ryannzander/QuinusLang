# Create a portable QuinusLang zip - no install, just extract and run
# Usage: .\make-portable.ps1
# Requires: LLVM (for building), clang (for runtime)
# Packages: quinus.exe, runtime.obj, lld-link.exe

$ErrorActionPreference = "Stop"

Write-Host "Building QuinusLang (Rust)..."
cargo build --release
if ($LASTEXITCODE -ne 0) { exit 1 }
Copy-Item "target\release\quinus.exe" "quinus.exe" -Force

# Build runtime
Write-Host "Building runtime..."
& "$PSScriptRoot\scripts\build-runtime.ps1"
if ($LASTEXITCODE -ne 0) { exit 1 }
if (Test-Path "dist-runtime\runtime.obj") {
    Copy-Item "dist-runtime\runtime.obj" "runtime.obj" -Force
}

# Copy lld-link from LLVM if available
$llvmPath = $env:LLVM_SYS_181_PREFIX
if (-not $llvmPath) { $llvmPath = $env:LLVM_SYS_170_PREFIX }
if (-not $llvmPath) { $llvmPath = "C:\Program Files\LLVM" }
$lldLink = Join-Path $llvmPath "bin\lld-link.exe"
if (Test-Path $lldLink) {
    Copy-Item $lldLink "lld-link.exe" -Force
} else {
    Write-Warning "lld-link.exe not found at $lldLink - portable zip may not link. Install LLVM."
}

$portableDir = "QuinusLang-portable"
$zipName = "QuinusLang-portable.zip"

if (Test-Path $portableDir) { Remove-Item $portableDir -Recurse -Force }
New-Item -ItemType Directory -Path $portableDir | Out-Null

Copy-Item "quinus.exe" "$portableDir\"
if (Test-Path "runtime.obj") { Copy-Item "runtime.obj" "$portableDir\" }
if (Test-Path "lld-link.exe") { Copy-Item "lld-link.exe" "$portableDir\" }
Copy-Item "stdlib" "$portableDir\" -Recurse
Copy-Item "compiler" "$portableDir\" -Recurse

# Add a simple README for portable users
@"
QuinusLang - Portable (LLVM backend)

Run: .\quinus.exe --help

No C compiler required. The compiler uses LLVM and lld for linking.
If lld-link.exe and runtime.obj are in this folder, they will be used automatically.

Then: quinus build yourfile.q
"@ | Out-File "$portableDir\README.txt" -Encoding utf8

if (Test-Path $zipName) { Remove-Item $zipName -Force }
Compress-Archive -Path $portableDir -DestinationPath $zipName

Remove-Item $portableDir -Recurse -Force
Write-Host "Created $zipName - extract anywhere and run quinus.exe"
