# Compilation (LLVM)

QuinusLang compiles directly to native executables via LLVM. There is no C backend—LLVM is the sole codegen path.

**No separate LLVM download for end users.** The installer and portable zip include everything:
- **quinus.exe** — LLVM codegen is built in (statically linked); no LLVM DLLs required
- **lld-link.exe** (Windows) / **ld.lld** (Linux) — Bundled linker
- **runtime.obj** / **runtime.o** — Precompiled stdlib helpers

One download, no additional tools.

## For End Users (Installer/Portable)

If you downloaded the installer or portable zip: **no additional setup required**. Run `quinus build` and it will compile your `.q` files to executables using the bundled tools.

## For Developers (Building from Source)

### Prerequisites

1. **LLVM 18** — Development libraries must be installed.
   - **Windows**: Download the official tarball from [llvm.org](https://llvm.org/) or `choco install llvm`; set `LLVM_SYS_181_PREFIX` to the install path
   - **macOS**: `brew install llvm@18`
   - **Linux**: `apt install llvm-18-dev` (or equivalent)

2. **Build the compiler**:
   ```bash
   cargo build --release
   ```

3. **Linking** — The compiler uses `lld` (LLVM's linker). Either:
   - Use the installer/portable zip which bundles `lld-link.exe` (Windows) or `ld.lld` (Linux)
   - Or have LLVM in PATH so `lld-link` / `ld.lld` can be found

4. **Runtime** (optional) — For programs using `str`, `vec`, `fmt`, etc., build the runtime:
   ```bash
   .\scripts\build-runtime.ps1   # Windows
   ./scripts/build-runtime.sh   # Linux (if available)
   ```
   Place `runtime.obj` (Windows) or `runtime.o` (Linux) next to `quinus.exe` or in `dist-runtime/`.

## Usage

```bash
quinus build
```

Or for a specific file:

```bash
quinus build path/to/main.q
```

Emit LLVM IR only (no linking):

```bash
quinus build --emit-llvm
```

## Supported Features

- **Types**: `i32`, `i64`, `f32`, `f64`, `bool`, `void`, `str`, pointers
- **Functions**: Parameters, return values, modules
- **Expressions**: Literals (int, float, bool, str), binary ops (including `%`, `&&`, `||`), unary (`-`, `!`), variables, function calls, `print`/`write`/`writeln`
- **Statements**: Variable declarations, assignments, `if`/`otherwise`, `send` (return), `for`, `loopwhile`, `defer`, `with`, expression statements

**Not yet supported**: `choose`, `Result` type, `ArrayInit`, `Foreach`, `TryCatch`, structs/classes, `InlineC`, and others.

## Pipeline

```
.q source → Parse → Semantic → LLVM IR → object.o → lld + runtime → executable
```
