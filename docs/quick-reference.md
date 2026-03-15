# Quick Reference

One-page cheat sheet for QuinusLang.

## Project Setup

```bash
quinus init
quinus run
```

## Variables

```q
make x: i32 = 42;        // Immutable
make shift y: i32 = 0;   // Mutable
y = y + 1;
```

## Functions

```q
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}
```

## Control Flow

```q
check (x > 0) { ... }
otherwise { ... }

loopwhile (i < 10) { ... }
foreach item in arr { ... }
for (make i = 0; i < 10; i = i + 1) { ... }
```

## Types

```q
i32, u64, f64, bool, str, void
link T, [T; N], (A, B), Result(Ok, Err)
```

## Structs and Enums

```q
form Point { x: i32, y: i32 }
state Color { Red, Green, Blue }
state Option(T) { None, Some(T) }
```

## Modules

```q
bring "std.fs";
bring "std.math";
fs.open_file("x.txt", "r");
```

## Pointers

```q
make p: link i32 = mark x;
make v: i32 = reach p;
```

## Unsafe

```q
hazard {
    cblock { " raw C " };
    machine { "asm" };
}
```

## CLI

```bash
quinus build [--release] [--emit-c] [--define NAME]
quinus run
quinus fmt
quinus watch
quinus check
quinus lsp
```
