# Keywords Reference

Complete reference for Q++ keywords and their usage.

## Variables and Bindings

| Keyword | Purpose | Example |
|---------|---------|---------|
| `make` | Immutable binding | `make x: i32 = 42;` |
| `make shift` | Mutable variable | `make shift i: i32 = 0;` |
| `eternal` | Compile-time constant | `eternal PI: f64 = 3.14159;` |
| `anchor` | Static variable | `anchor count: i32 = 0;` |

## Functions

| Keyword | Purpose | Example |
|---------|---------|---------|
| `craft` | Define function | `craft add(a: i32, b: i32) -> i32 { ... }` |
| `send` | Return from function | `send x + y;` |
| `extern craft` | C FFI declaration | `extern craft malloc(size: usize) -> link void;` |

## Control Flow

| Keyword | Purpose | Example |
|---------|---------|---------|
| `check` | If condition | `check (x > 0) { ... }` |
| `otherwise` | Else branch | `otherwise { ... }` |
| `loopwhile` | While loop | `loopwhile (i < 10) { ... }` |
| `foreach` | For-each loop | `foreach item in arr { ... }` |
| `for` | C-style for loop | `for (init; cond; step) { ... }` |
| `stop` | Break from loop | `stop;` |
| `skip` | Continue to next iteration | `skip;` |

## Types and Structures

| Keyword | Purpose | Example |
|---------|---------|---------|
| `form` | Struct definition | `form Point { x: i32, y: i32 }` |
| `state` | Enum definition | `state Color { Red, Green, Blue }` |
| `fusion` | Union definition | `fusion Maybe { value: i32 }` |
| `alias` | Type alias | `alias Id = u64;` |
| `impl` | Struct methods | `impl Point { craft magnitude(self) -> f64 { ... } }` |

## Modules

| Keyword | Purpose | Example |
|---------|---------|---------|
| `realm` | Module definition | `realm math { ... }` |
| `bring` | Import module | `bring "std.fs";` |

## Pointers and Memory

| Keyword | Purpose | Example |
|---------|---------|---------|
| `link` | Pointer type | `link i32`, `link void` |
| `mark` | Address-of | `mark x` |
| `reach` | Dereference | `reach p` |

## Unsafe and Low-Level

| Keyword | Purpose | Example |
|---------|---------|---------|
| `hazard` | Unsafe block | `hazard { ... }` |
| `machine` | Inline assembly | `machine { "mov eax, 1" }` |
| `cblock` | Inline C code | `cblock { " int x = 0; " }` |

## Other

| Keyword | Purpose | Example |
|---------|---------|---------|
| `defer` | Run at scope exit | `defer { cleanup(); }` |
| `with` | Scope-bound resource | `with f = fs.open_file(...) { ... }` |
| `choose` | Pattern matching | `choose x { Ok(v) => ... }` |
| `move` | Move expression | `make y = move x;` |
