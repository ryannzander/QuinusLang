# Installation

Choose the method that fits your workflow.

## Windows Installer

Download `Q++-Setup.exe` from [releases](https://github.com/ryannzander/Q++/releases).

1. Run the installer
2. Check "Add to PATH" (recommended)
3. Run `qpp` from any terminal

The installer bundles everything: LLVM is built into qpp.exe, plus the linker (`lld-link.exe`) and runtime. No separate LLVM or C compiler download required.

## Portable Zip

Download `Q++-portable.zip` from [releases](https://github.com/ryannzander/Q++/releases).

1. Extract anywhere
2. Run `qpp.exe` from that folder
3. No admin rights or PATH changes required

The portable zip includes qpp.exe (with LLVM built in), `lld-link.exe`, and `runtime.obj`. Extract and run—no additional tools needed.

## From Source

Requires **Rust** and **LLVM 18** (for building the compiler).

```powershell
# Windows
choco install llvm
git clone https://github.com/ryannzander/Q++.git
cd Q++
cargo build --release
```

```bash
# Linux / macOS
# Install LLVM 18 (e.g. apt install llvm-18-dev or brew install llvm@18)
git clone https://github.com/ryannzander/Q++.git
cd Q++
cargo build --release
```

## Runtime (for str, vec, fmt, etc.)

When building from source, compile the runtime for programs that use stdlib modules:

```powershell
# Windows
.\scripts\build-runtime.ps1
```

```bash
# Linux
./scripts/build-runtime.sh
```

Place `runtime.obj` (Windows) or `runtime.o` (Linux) in `dist-runtime/` or next to `qpp.exe`.
