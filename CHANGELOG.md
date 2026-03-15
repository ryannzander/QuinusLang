# Changelog

All notable changes to QuinusLang will be documented in this file.

## [Unreleased]

- (none)

## [0.2.6] - 2025-03-14

### Fixed

- **Linux linking**: Use clang as linker fallback when ld.lld cannot find libc (-lc); fixes CI and Linux builds

## [0.2.2] - 2025-03-14

### Added

- **stdlib expansion**: std.hash (FNV-1a, djb2), std.mem (copy, set, compare), std.term (ANSI colors/cursor), std.sys (is_windows, is_unix), std.gui (Raylib)
- **with blocks**: scope-bound resources with automatic cleanup; `with f = fs.open_file(...) { ... }` closes file on block exit
- **Benchmark suite**: benchmarks/ with sum, fib, sieve, mandelbrot, nbody in QuinusLang, C, Rust, Zig; run.ps1 for timing
- **VSCode syntax**: updated keywords (with, defer, choose, alias, extern, move, cblock)

## [0.2.1] - 2025-03-14

### Changed

- cargo fmt compliance, clippy warnings fixed, dead code cleanup

## [0.2.0] - 2025-03-14

### Added

- **Bitfields**: struct fields with `field: u32 : 8` syntax for packed bit storage
- **Move semantics**: `move x` expression; semantic pass tracks use-after-move
- **Inline C**: `cblock { " raw C " }` inside hazard blocks for low-level code
- **stdlib/arena.q**: arena.alloc(size), arena.dealloc(ptr) wrapping malloc/free
- **stdlib/simd.q**: SSE wrappers (loadu_ps, storeu_ps, add_ps, mul_ps)
- **stdlib/time.q**: time.now_us(), time.sleep_ms()
- **stdlib/rand.q**: rand.u32(), rand.range(min, max)
- **Compile flags**: `#if`, `#else`, `#endif`, `#define`; `-DNAME=val` CLI
- **Checked arithmetic**: math.add_checked, sub_checked, mul_checked
- **String interpolation**: `` `Hello, ${name}!` `` — backtick strings with `${expr}`
- **Docs**: stdlib index, tour, types, structs, hazard, FFI, compile-flags, embedded

### Changed

- stdlib/math.q: add checked arithmetic helpers

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

## [0.1.0] - Initial Release

- Lexer, parser, semantic analysis
- C code generation
- Variables, functions, control flow
- Structs, enums, unions
- Pointers, arrays, strings
- Modules (realm), imports (bring)
- Hazard blocks, defer, choose (pattern matching)
- Enum payloads, struct methods (impl)
