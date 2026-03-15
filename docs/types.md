# Types

## Primitive Types

| Type | Description |
|------|-------------|
| `i8`, `i16`, `i32`, `i64` | Signed integers |
| `u8`, `u16`, `u32`, `u64` | Unsigned integers |
| `usize` | Unsigned size |
| `f32`, `f64` | Floats |
| `bool` | Boolean |
| `str` | String (char*) |
| `void` | Unit type |

## Compound Types

| Type | Example |
|------|---------|
| Pointer | `link i32`, `link str` |
| Array (fixed) | `[i32; 5]` |
| Array (unsized) | `[i32]` |
| Tuple | `(i32, str)` |

## Type Aliases

```q
alias Id = u64;
alias Buffer = [u8; 256];
```

## Cast

```q
make x: i32 = 42;
make n: usize = x as usize;
```
