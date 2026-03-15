# QuinusLang Specification

Assembly Power with Readable Modern Syntax

QuinusLang is a modern low-level systems programming language designed to provide assembly-level control with a clean, readable syntax.

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

## Builtins

- `print(...)` — Output to stdout via printf

## File Extension

`.q` (per project decision; spec originally used `.quin`)
