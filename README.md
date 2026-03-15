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

# Compile a program
quinus build
quinus build path/to/file.quin

# Build and run
quinus run

# Parse only (debug)
quinus parse file.quin

# Package manager
quinus add <package>
quinus add <package> --git <url>
quinus remove <package>
```

## Language Syntax

### Variables
```quin
var x: int = 42;
var y = 10;
```

### Control Flow
```quin
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
```quin
fn add(a: int, b: int) -> int {
    return a + b;
}
```

### Structs
```quin
struct Point {
    x: int,
    y: int
}
```

### Classes
```quin
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
- NASM or MSVC (for assembling output)
