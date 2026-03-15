# Changelog

All notable changes to QuinusLang will be documented in this file.

## [Unreleased]

### Added

- String interpolation: `` `Hello, ${name}!` `` — backtick strings with `${expr}` for print/write/writeln

## [0.1.1] - 2025-03-14

### Added

- Automated GitHub Releases: push tag `vX.Y.Z` to trigger release workflow
- Semantic error spans: `semantic_err_span` with line/col for Const/Static type errors
- LSP: real hover (identifier type lookup), diagnostics on save
- CI: `cargo fmt --check` and `cargo clippy`
- Watch debounce (300ms) to avoid redundant rebuilds
- Error-case tests: type mismatch, wrong arg count, const type
- Formatter round-trip tests: defer, foreach
- Stdlib tests: fs, math modules

### Changed

- Fixed outdated cmd_run message (C backend, not NASM)
- Fixed README: `quinus publish` description
- LSP version from Cargo.toml
- REPL type display uses Display instead of Debug

### Removed

- (none)
- stdlib/math.q: abs_i32, abs_f64, min_i32, max_i32, min_f64, max_f64, sqrt_f64
- stdlib/os.q: os.getenv(name), os.cwd() for environment and current directory
- Git package fetch: dependencies with `git = "url"` are cloned before build
- Formatter: support for For, Foreach, Defer, Choose, Hazard, TryCatch, InlineAsm, Assign
- `quinus check [path]` — parse + semantic only, no codegen
- Phase 2.4: Improved semantic errors with "Did you mean?" hints
- stdlib/str.q: trim, concat
- `quinus publish` — validate and create Git tag
- REPL: type inference display on success
- `quinus lsp` — Language Server Protocol for IDE support

## [0.2.0] - 2025-03-14

### Added

- C FFI: `extern craft` declarations for calling C library functions
- stdlib/fs.q: File I/O (open_file, close, read_all, exists, write_all)
- stdlib/os.q: Process execution (os.run)
- Tuple destructuring: `make (a, b) = div_rem(17, 5);`
- Type alias resolution in codegen
- Cast expression: `expr as type` (e.g. `x as usize`)
- Module function mangling for realm/namespace support

### Changed

- ARCHITECTURE.md updated to describe C backend (not NASM)
- SPEC.md updated with alias, extern, defer, choose, impl, stdlib

### Added (docs)

- CONTRIBUTING.md — Build, test, PR process
- LICENSE — MIT

## [0.1.0] - Initial Release

- Lexer, parser, semantic analysis
- C code generation
- Variables, functions, control flow
- Structs, enums, unions
- Pointers, arrays, strings
- Modules (realm), imports (bring)
- Hazard blocks, defer, choose (pattern matching)
- Enum payloads, struct methods (impl)
