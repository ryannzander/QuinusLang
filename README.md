# QuinusLang

A systems programming language with assembly-level control and readable syntax. Compiles to C, then to native executables via your system compiler.

## Philosophy

- **Explicit control** — Expensive or dangerous operations are visible in code
- **Readability** — Low-level code should be easy to read
- **Assembly-level power** — Inline assembly, pointers, zero-cost abstractions
- **Safety by design** — Unsafe operations require explicit `hazard` blocks
- **Zero hidden runtime** — Suitable for kernels, bootloaders, firmware, embedded

---

## Installation

### From source

```bash
git clone https://github.com/ryannzander/QuinusLang.git
cd QuinusLang
cargo build --release
```

### Install globally (add to PATH)

```bash
cargo install --path .
```

This installs `quinus` to `~/.cargo/bin/` (already on PATH if you have Rust).

---

## Quick Start

```bash
# Create a new project
quinus init

# Compile and run
quinus run

# Or build only
quinus build
```

Your entry point is `src/main.q`:

```q
craft main() -> void {
    print(42);
    print("Hello, QuinusLang!");
    send;
}
```

---

## CLI Reference

| Command | Description |
|---------|-------------|
| `quinus build [path]` | Compile to executable (default: current dir) |
| `quinus build --release` | Optimized build |
| `quinus build --emit-c` | Emit C only, do not compile |
| `quinus run [path]` | Build and run |
| `quinus run --release` | Run release build |
| `quinus init [path]` | Create new package |
| `quinus parse <file>` | Parse and dump AST (debug) |
| `quinus fmt [path]` | Format .q files |
| `quinus watch [path]` | Rebuild on file changes |
| `quinus repl` | Interactive REPL (parse & show AST) |
| `quinus add <pkg>` | Add dependency |
| `quinus add <pkg> --git <url>` | Add package from Git |
| `quinus remove <pkg>` | Remove dependency |
| `quinus update` | Update dependencies |
| `quinus publish` | Publish to registry (not implemented) |

---

## Language Reference

### File extension

`.q`

### Variables

```q
make x: i32 = 42;        // immutable
make shift y: i32 = 10;  // mutable
```

### Control flow

```q
check (x > 0) {
    // then
}
otherwise {
    // else
}

loopwhile (i < 10) {
    i = i + 1;
}

for (make shift i: i32 = 0; i < 10; i = i + 1) {
    print(i);
}

foreach item in arr {
    print(item);
    stop;   // break
    skip;   // continue
}
```

### Functions

```q
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}
```

### Structs

```q
form Point {
    x: i32,
    y: i32
}

make shift p: Point = ...;
make shift px: i32 = p.x;
```

### Enums and unions

```q
state Color {
    Red,
    Green,
    Blue
}

fusion Maybe {
    value: i32
}
```

### Pointers

```q
make shift p: link i32 = mark x;   // address-of
reach p = 99;                      // dereference and assign
make shift v: i32 = reach p;       // dereference
```

### Arrays

```q
make shift arr: [i32; 5] = { 1, 2, 3, 4, 5 };
make shift n: usize = len(arr);
make shift x: i32 = arr[0];

// Slices (produce pointer to subarray)
arr[1..4]   // from index 1 to 4
arr[..3]    // from start to 3
arr[2..]    // from 2 to end
```

### Strings

```q
make shift s: str = "hello";
make shift t: str = s + " world";
make shift n: usize = strlen(s);
print(s);
```

### Modules and imports

```q
bring "std.io";

realm math {
    craft add(a: i32, b: i32) -> i32 {
        send a + b;
    }
}
```

### Unsafe and inline assembly

```q
hazard {
    machine { "mov eax, 1" };
}
```

### Constants and statics

```q
eternal PI: f64 = 3.14159;
anchor counter: i32 = 0;
```

---

## Builtins

| Builtin | Description |
|---------|-------------|
| `print(...)` | Print to stdout with newline |
| `write(...)` | Print to stdout without newline |
| `writeln(...)` | Print to stdout with newline |
| `read()` | Read integer from stdin |
| `len(arr)` | Array length (array or `[T; N]`) |
| `strlen(s)` | String length |
| `panic()` | Abort with message |
| `assert(cond)` | Abort if condition is false |

---

## Types

| Type | Description |
|------|-------------|
| `i8`, `i16`, `i32`, `i64` | Signed integers |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers |
| `usize` | Unsigned size |
| `f32`, `f64` | Floats |
| `bool` | Boolean |
| `str` | String (char*) |
| `void` | Unit type |
| `int`, `float` | Legacy aliases |
| `link T` | Pointer to T |
| `[T; N]` | Fixed-size array |
| `[T]` | Array (unsized) |

---

## Project layout

```
my-project/
├── quinus.toml      # Manifest (optional)
├── src/
│   └── main.q       # Entry point
├── stdlib/          # Optional: local stdlib
└── build/           # Output (generated)
    ├── output.c
    └── output.exe
```

### quinus.toml

```toml
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
# foo = "0.1"

[build]
entry = "src/main.q"
```

---

## Requirements

- **Rust** — To build the compiler
- **C compiler** — MinGW (`winget install mingw`), MSVC Build Tools, or Clang

---

## Compiler pipeline

```
.q source → Lexer → Parser → AST → Semantic → C code → System compiler → .exe
```

---

## Project structure (compiler)

```
src/
├── lexer/     # Tokenization
├── parser/    # AST parsing
├── ast/       # AST definitions
├── semantic/  # Type checking, symbol tables
├── codegen/   # C emission
├── fmt/       # Formatter
└── package/   # Package manager
```

---

## Syntax highlighting

Install the `syntax/` extension for Cursor/VS Code:

1. `Ctrl+Shift+P` → **Developer: Install Extension from Location...**
2. Select the `syntax/` folder

---

## Documentation

Docs are built with [MkDocs Material](https://squidfunk.github.io/mkdocs-material/) and deployed to GitHub Pages:

1. Edit `mkdocs.yml` — set `site_url` to your Pages URL (e.g. `https://ryannzander.github.io/QuinusLang/`)
2. Push to GitHub — the `Deploy docs` workflow builds and deploys to the `gh-pages` branch
3. Enable Pages — Repo → Settings → Pages → Source: Deploy from branch → Branch: `gh-pages`

Local preview: `pip install mkdocs-material && mkdocs serve`

---

## License

[Add your license here]
