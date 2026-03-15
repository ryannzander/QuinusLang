# QuinusLang Architecture

## Compiler Pipeline

1. **Lexer** - Tokenizes source into tokens (identifiers, keywords, literals, operators)
2. **Parser** - Builds AST from token stream (recursive descent)
3. **Semantic Analysis** - Type checking, symbol resolution
4. **Code Generator** - Emits C code (primary backend)

## Backend

- **C backend** - Transpiles to portable C, compiled with system compiler (GCC, Clang, MSVC)
- Emits C source to `build/output.c`, then invokes the C compiler to produce executables

## Package Manager

- `quinus.toml` - Manifest with package metadata and dependencies
- `quinus.lock` - Lock file for reproducible builds
- Resolves dependencies from registry or Git

## Output

- Compiles to C, then to native executables via system C compiler
- No NASM or separate linker required
