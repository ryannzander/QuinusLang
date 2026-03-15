# QuinusLang Architecture

## Compiler Pipeline

1. **Lexer** - Tokenizes source into tokens (identifiers, keywords, literals, operators)
2. **Parser** - Builds AST from token stream (recursive descent)
3. **Semantic Analysis** - Type checking, symbol resolution
4. **Code Generator** - Emits x86/x64 NASM assembly

## Package Manager

- `quinus.toml` - Manifest with package metadata and dependencies
- `quinus.lock` - Lock file for reproducible builds
- Resolves dependencies from registry or Git

## Output

- Compiles to NASM assembly (Windows x64)
- Requires NASM and linker to produce executables
