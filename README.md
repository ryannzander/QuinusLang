# QuinusLang

A fully functional compilable programming language with rich features: variables, control flow, functions, arrays, structs, classes, modules, and a package manager.

## Building

```bash
cargo build
```

## Usage

```bash
# Create a new package
quinus init

# Compile to .exe (requires NASM + MinGW GCC)
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
```

## Language Syntax

### Variables
```q
var x: int = 42;
var y = 10;
```

### Control Flow
```q
if (x > 0) {
    // ...
}

while (i < 10) {
    i = i + 1;
}

for (var i = 0; i < 10; i = i + 1) {
    // ...
}
```

### Functions
```q
func add(a: int, b: int) -> int {
    return a + b;
}
```

### Structs
```q
struct Point {
    x: int,
    y: int
}
```

### Classes
```q
class Point {
    x: int
    y: int

    init(x: int, y: int) {
        this.x = x;
        this.y = y;
    }
}

var p: Point = new Point(1, 2);
```

## Project Structure

- `src/lexer/` - Tokenization
- `src/parser/` - AST parsing
- `src/ast/` - AST definitions
- `src/semantic/` - Type checking, symbol tables
- `src/codegen/` - x86/x64 assembly emission
- `src/package/` - Package manager

## Requirements

- Rust (for building the compiler)
- **NASM**: https://nasm.us/
- **MinGW GCC** or **MSVC** (for linking)

`quinus build` compiles to machine code: .q → assembly → NASM → .obj → link → .exe
