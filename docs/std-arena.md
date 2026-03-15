# std.arena

Simple allocator wrappers (malloc/free).

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `alloc` | `(size: usize) -> link void` | Allocate bytes |
| `dealloc` | `(ptr: link void) -> void` | Free allocation |

## Example

```q
bring "std.arena";

craft main() -> void {
    make p: link void = arena.alloc(64);
    arena.dealloc(p);
    send;
}
```
