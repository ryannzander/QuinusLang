# QuinusLang

A systems programming language with assembly-level control and readable syntax. Compiles to C, then to native executables.

## Building

```bash
cargo build
```

## Usage

```bash
# Create a new package
quinus init

# Compile to executable (requires C compiler: MinGW, MSVC, or Clang)
quinus build
quinus build path/to/file.q

# Build and run
quinus run

# Parse only (debug)
quinus parse file.q

# Package manager
quinus add <package>
quinus add <package> --git <url>
quinus remove <package>
quinus update

# Format source
quinus fmt
```

## Language Syntax (Spec)

### Variables
```q
make x: i32 = 42;        // immutable
make shift y: i32 = 10;  // mutable
```

### Control Flow
```q
check (x > 0) {
    // ...
}
otherwise {
    // ...
}

loopwhile (i < 10) {
    i = i + 1;
}

foreach item in collection {
    // stop; skip;
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
```

### Pointers
```q
make shift p: link i32 = mark x;
reach p = 99;
```

### Builtins
- `print(...)` — Output to stdout (with newline)
- `write(...)` — Output to stdout (no newline)
- `writeln(...)` — Output to stdout (with newline)
- `read()` — Read integer from stdin
- `len(arr)` — Array length

## Project Structure

- `src/lexer/` — Tokenization
- `src/parser/` — AST parsing
- `src/ast/` — AST definitions
- `src/semantic/` — Type checking, symbol tables
- `src/codegen/` — C code emission
- `src/package/` — Package manager

## Requirements

- Rust (for building the compiler)
- C compiler: MinGW (`winget install mingw`), MSVC Build Tools, or Clang

`quinus build` compiles: .q → AST → C → native executable
