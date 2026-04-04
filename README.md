<p align="center">
  <img src="assets/logo-brand.svg" alt="Q++" width="220" />
</p>

<h1 align="center">Q++</h1>

<p align="center">
  <strong>A systems programming language with assembly-level control, readable syntax, and an LLVM-powered native compiler.</strong>
</p>

<p align="center">
  <a href="https://github.com/ryannzander/QuinusLang/releases"><img src="https://img.shields.io/github/v/release/ryannzander/QuinusLang?style=flat-square&color=7C3AED" alt="Release" /></a>
  <a href="https://github.com/ryannzander/QuinusLang/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/ryannzander/QuinusLang/ci.yml?branch=master&style=flat-square&label=CI" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/ryannzander/QuinusLang?style=flat-square" alt="License" /></a>
</p>

<p align="center">
  <a href="https://ryannzander.github.io/QuinusLang/">Documentation</a> &middot;
  <a href="https://github.com/ryannzander/QuinusLang/releases">Downloads</a> &middot;
  <a href="#quick-start">Quick Start</a>
</p>

---

## Why Q++?

- **Explicit control** — Expensive or dangerous operations are visible in code
- **Readability** — Low-level code should be easy to read
- **Assembly-level power** — Inline assembly, pointers, zero-cost abstractions
- **Safety by design** — Unsafe operations require explicit `hazard` blocks
- **Zero hidden runtime** — Suitable for kernels, bootloaders, firmware, embedded
- **LLVM backend** — Compiles `.q` files to native executables via LLVM, no C compiler required

---

## Install

| Option | How |
|--------|-----|
| **Installer** (recommended) | Download `Q++-Setup.exe` from [Releases](https://github.com/ryannzander/QuinusLang/releases) — run it, check "Add to PATH", done. |
| **Portable zip** | Download `Q++-portable.zip` — extract anywhere, run `qpp.exe`. No admin needed. |
| **From source** | `git clone https://github.com/ryannzander/QuinusLang.git && cd QuinusLang && cargo build --release` (requires Rust + LLVM 18) |

Both the installer and portable zip bundle `lld-link` so you can compile `.q` files without installing any other toolchain.

---

## Quick Start

```bash
qpp init my-app      # scaffold a new project
cd my-app
qpp run              # compile and execute
```

Your entry point is `src/main.q`:

```q
craft main() -> void {
    print("Hello, Q++!");
    send;
}
```

Build only (output goes to `build/output.exe`):

```bash
qpp build
qpp build --release   # optimized
```

---

## Language at a Glance

### Variables

```q
make x: i32 = 42;           // immutable
make shift y: i32 = 10;     // mutable
eternal PI: f64 = 3.14159;  // constant
anchor counter: i32 = 0;    // static
```

### Functions

```q
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}
```

### Control Flow

```q
check (x > 0) {
    // then
} otherwise {
    // else
}

loopwhile (i < 10) { i = i + 1; }

for (make shift i: i32 = 0; i < 10; i = i + 1) {
    print(i);
}

foreach item in arr {
    print(item);
    stop;    // break
    skip;    // continue
}
```

### Structs, Enums & Unions

```q
form Point { x: i32, y: i32 }

state Color { Red, Green, Blue }

fusion Maybe { value: i32 }
```

### Pointers

```q
make shift p: link i32 = mark x;   // address-of
reach p = 99;                       // deref-assign
make shift v: i32 = reach p;       // deref-read
```

### Arrays & Strings

```q
make shift arr: [i32; 5] = { 1, 2, 3, 4, 5 };
make shift s: str = "hello" + " world";
```

### Modules

```q
bring "std.io";

realm math {
    craft square(n: i32) -> i32 { send n * n; }
}
```

### Unsafe & Inline Assembly

```q
hazard {
    machine { "mov eax, 1" };
}
```

---

## Standard Library

Q++ ships with a growing stdlib in `stdlib/`:

| Module | Purpose |
|--------|---------|
| `io` | Console I/O (`print`, `read`, `write`) |
| `fs` | File system (open, read, write, close) |
| `os` | OS interaction, environment variables |
| `str` | String manipulation |
| `vec` | Dynamic arrays |
| `map` | Hash maps |
| `math` | Math functions |
| `fmt` | String formatting |
| `rand` | Random number generation |
| `time` | Timestamps and timing |
| `mem` | Memory allocation (malloc, free, arena) |
| `arena` | Arena allocator |
| `hash` | Hashing utilities |
| `path` | File path manipulation |
| `sys` | Low-level system calls |
| `term` | Terminal colors and control |
| `simd` | SIMD intrinsics |
| `gui` | GUI primitives |

Import with `bring "std.fs";` etc.

---

## Type System

| Type | Description |
|------|-------------|
| `i8` `i16` `i32` `i64` | Signed integers |
| `u8` `u16` `u32` `u64` | Unsigned integers |
| `usize` | Unsigned pointer-size |
| `f32` `f64` | Floating point |
| `bool` | Boolean |
| `str` | String (`char*`) |
| `void` | Unit |
| `link T` | Pointer to `T` |
| `[T; N]` | Fixed-size array |

---

## CLI Reference

| Command | Description |
|---------|-------------|
| `qpp build [path]` | Compile to native executable |
| `qpp build --release` | Optimized build |
| `qpp build --emit-llvm` | Emit LLVM IR only |
| `qpp run [path]` | Build and run |
| `qpp check [path]` | Parse and type-check without compiling |
| `qpp init [name]` | Scaffold a new project |
| `qpp fmt [path]` | Format `.q` source files |
| `qpp watch [path]` | Rebuild on file changes |
| `qpp repl` | Interactive REPL |
| `qpp add <pkg>` | Add dependency |
| `qpp remove <pkg>` | Remove dependency |
| `qpp update` | Update dependencies |
| `qpp publish` | Validate and tag a release |

---

## Compiler Architecture

```
.q source
  → Preprocessor (bring/flatten)
  → Lexer (logos)
  → Parser → AST
  → Semantic analysis (type checking, symbol resolution)
  → LLVM IR codegen (inkwell)
  → Native object code (LLVM)
  → Linker (lld-link / ld.lld)
  → Executable
```

The compiler is written in Rust and uses LLVM 18 via [inkwell](https://github.com/TheDan64/inkwell). A self-hosting bootstrap compiler written in Q++ itself lives in `compiler/`.

### Project Structure

```
src/
├── lexer/      Tokenization (logos)
├── parser/     Recursive-descent parser → AST
├── ast/        AST node definitions
├── semantic/   Type checking, scopes, symbol tables
├── codegen/    LLVM IR emission (inkwell)
├── fmt/        Source formatter
└── package/    Package manager & dependency resolution

stdlib/         Standard library (.q modules)
runtime/        C runtime linked into every executable
compiler/       Self-hosting bootstrap compiler (.q)
```

---

## Syntax Highlighting

Install the VS Code / Cursor extension from the `syntax/` folder:

1. `Ctrl+Shift+P` → **Developer: Install Extension from Location...**
2. Select the `syntax/` directory

---

## Documentation

Full language reference, guides, and API docs:

**[https://ryannzander.github.io/QuinusLang/](https://ryannzander.github.io/QuinusLang/)**

Built with [MkDocs Material](https://squidfunk.github.io/mkdocs-material/) and deployed automatically via GitHub Actions.

---

## License

MIT — see [LICENSE](LICENSE)
