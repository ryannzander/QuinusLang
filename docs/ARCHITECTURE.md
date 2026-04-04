# Q++ Architecture

## Compiler Pipeline

1. **Lexer** — Tokenizes source into tokens (identifiers, keywords, literals, operators)
2. **Parser** — Builds AST from token stream (recursive descent)
3. **Semantic Analysis** — Type checking, symbol resolution
4. **Code Generator** — Emits LLVM IR, then object code

## Backend

- **LLVM backend** — Compiles directly to machine code via LLVM IR
- Emits object file to `build/output.o`, then links with `lld` (LLVM linker) and runtime
- No C compilation; no gcc/clang/MSVC required for end users
- Bundled `lld-link.exe` (Windows) or `ld.lld` (Linux) for linking

## Runtime

- Precompiled `runtime.obj` / `runtime.o` provides stdlib helpers (str, vec, fmt, etc.)
- Built from C sources in `runtime/` at release time; shipped with installer/portable
- Linked with user code at compile time

## Package Manager

- `qpp.toml` — Manifest with package metadata and dependencies
- `qpp.lock` — Lock file for reproducible builds
- Resolves dependencies from registry or Git

## Output

- Compiles to native executables via LLVM (object file → lld → executable)
- Optional `--emit-llvm` outputs `.ll` IR for debugging
