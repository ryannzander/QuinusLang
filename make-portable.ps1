# Create a portable QuinusLang zip - no install, just extract and run
# Usage: .\make-portable.ps1

$ErrorActionPreference = "Stop"

Write-Host "Building QuinusLang..."
& "$PSScriptRoot\build.ps1"
if ($LASTEXITCODE -ne 0) { exit 1 }

$portableDir = "QuinusLang-portable"
$zipName = "QuinusLang-portable.zip"

if (Test-Path $portableDir) { Remove-Item $portableDir -Recurse -Force }
New-Item -ItemType Directory -Path $portableDir | Out-Null

Copy-Item "quinus.exe" "$portableDir\"
Copy-Item "stdlib" "$portableDir\" -Recurse
Copy-Item "compiler" "$portableDir\" -Recurse

# Add a simple README for portable users
@"
QuinusLang - Portable

Run: .\quinus.exe --help

To compile .q files you need a C compiler:
  winget install mingw
  or
  winget install Microsoft.VisualStudio.2022.BuildTools

Then: quinus build yourfile.q
"@ | Out-File "$portableDir\README.txt" -Encoding utf8

if (Test-Path $zipName) { Remove-Item $zipName -Force }
Compress-Archive -Path $portableDir -DestinationPath $zipName

Remove-Item $portableDir -Recurse -Force
Write-Host "Created $zipName - extract anywhere and run quinus.exe"
