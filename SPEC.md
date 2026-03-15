# QuinusLang Specification

Assembly Power with Readable Modern Syntax

QuinusLang is a modern low-level systems programming language designed to provide assembly-level control with a clean, readable syntax. It compiles to C, then to native executables via your system compiler.

## Core Philosophy

- **Explicit Control** — All expensive or dangerous operations are visible in code
- **Readability** — Low-level code should be readable
- **Assembly-Level Power** — Inline assembly, register control, memory-mapped I/O, etc.
- **Safety by Design** — Unsafe operations must be explicitly marked with `hazard`
- **Zero Hidden Runtime** — For kernels, bootloaders, firmware, embedded

## Keywords (Spec)

| Concept | Keyword |
|---------|---------|
| Function | `craft` |
| Return | `send` |
| Variable (immutable) | `make` |
| Variable (mutable) | `make shift` |
| Constant | `eternal` |
| Static | `anchor` |
| If | `check` |
| Else | `otherwise` |
| While | `loopwhile` |
| For-each | `foreach` |
| Break | `stop` |
| Continue | `skip` |
| Struct | `form` |
| Enum | `state` |
| Union | `fusion` |
| Module | `realm` |
| Pointer type | `link` |
| Address-of | `mark` |
| Dereference | `reach` |
| Unsafe block | `hazard` |
| Inline asm | `machine` |
| Type alias | `alias` |
| Import | `bring` |
| C FFI | `extern craft` |
| Defer | `defer` |
| Pattern match | `choose` |

## Language Features

- **Type aliases**: `alias Id = u64;`
- **C FFI**: `extern craft fopen(path: str, mode: str) -> link void;`
- **Tuple destructuring**: `make (a, b) = div_rem(17, 5);`
- **Cast expression**: `x as usize`
- **String interpolation**: `` `Hello, ${name}!` `` — backtick strings with `${expr}` (use with print/write/writeln)
- **Struct methods**: `impl` blocks with `craft name(self: Type, ...)`
- **Enum payloads**: `state Option { None, Some(T) }`
- **Defer**: `defer { ... }` runs at scope exit
- **Pattern matching**: `choose expr { ... }`

## Builtins

- `print(...)`, `write(...)`, `writeln()` — Output to stdout via printf
- `read()` — Read integer from stdin
- `len(arr)`, `strlen(s)` — Length
- `panic()`, `assert(cond)` — Abort

## Standard Library

- **io** — println wrapper
- **fs** — open_file, close, read_all, exists, write_all
- **os** — run (process execution)

## File Extension

`.q` (per project decision; spec originally used `.quin`)
