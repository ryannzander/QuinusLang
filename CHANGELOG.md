# Changelog

All notable changes to QuinusLang will be documented in this file.

## [Unreleased]

### Added

- String interpolation: `` `Hello, ${name}!` `` — backtick strings with `${expr}` for print/write/writeln
- stdlib/math.q: abs_i32, abs_f64, min_i32, max_i32, min_f64, max_f64, sqrt_f64
- stdlib/os.q: os.getenv(name), os.cwd() for environment and current directory
- Git package fetch: dependencies with `git = "url"` are cloned before build
- Formatter: support for For, Foreach, Defer, Choose, Hazard, TryCatch, InlineAsm, Assign
- `quinus check [path]` — parse + semantic only, no codegen

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
