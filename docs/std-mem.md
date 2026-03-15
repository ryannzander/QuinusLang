# std.mem

Memory utilities: thin wrappers over `memcpy`, `memset`, `memcmp`.

## Functions

| Function | Description |
|----------|-------------|
| `mem.copy(dest, src, n)` | Copy `n` bytes from `src` to `dest` |
| `mem.set(ptr, c, n)` | Set `n` bytes at `ptr` to value `c` |
| `mem.compare(a, b, n)` | Compare `n` bytes; returns 0 if equal, &lt;0 or &gt;0 otherwise |

## Example

```q
bring "std.mem";

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;

craft main() -> void {
    make buf: link void = malloc(16);
    mem.set(buf, 0, 16);
    make src: str = "hello";
    mem.copy(buf, src, 6);
    make cmp: i32 = mem.compare(buf, src, 5);
    free(buf);
    send;
}
```
