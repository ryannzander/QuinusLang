# C FFI

Q++ compiles to C, so calling C is straightforward.

## extern craft

Declare C functions:

```q
extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;
extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;
```

## Calling Conventions

- `str` maps to `char*`
- `link T` maps to `T*`
- Structs are passed by value; use `link StructName` for pointers
- Return types follow C ABI

## Passing Structs

```q
form Point { x: i32, y: i32 }

extern craft c_use_point(p: link Point) -> void;

craft main() -> void {
    make shift pt: Point = ...;
    c_use_point(mark pt);
    send;
}
```

## Common C Types

| Q++ | C |
|------------|---|
| `i32` | `int32_t` |
| `u64` | `uint64_t` |
| `str` | `char*` |
| `link void` | `void*` |
