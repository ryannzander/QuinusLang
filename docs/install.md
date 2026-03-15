# Installation

Choose the method that fits your workflow.

## Windows Installer

Download `QuinusLang-Setup.exe` from [releases](https://github.com/ryannzander/QuinusLang/releases).

1. Run the installer
2. Check "Add to PATH" (recommended)
3. Run `quinus` from any terminal

## Portable Zip

Download `QuinusLang-portable.zip` from [releases](https://github.com/ryannzander/QuinusLang/releases).

1. Extract anywhere
2. Run `quinus.exe` from that folder
3. No admin rights or PATH changes required

## From Source

Requires **Rust** and a **C compiler**.

```powershell
# Windows
git clone https://github.com/ryannzander/QuinusLang.git
cd QuinusLang
.\build.ps1
```

```bash
# Linux / macOS
git clone https://github.com/ryannzander/QuinusLang.git
cd QuinusLang
./build.sh
```

## C Compiler

To compile `.q` files, you need a C compiler:

| Platform | Command |
|----------|---------|
| Windows | `winget install mingw` or install MSVC Build Tools |
| macOS | Xcode Command Line Tools or `brew install gcc` |
| Linux | `apt install build-essential` or equivalent |
