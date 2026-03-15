# std.hash

Hashing utilities: FNV-1a and djb2.

## Functions

| Function | Description |
|----------|-------------|
| `hash.fnv1a(s, len)` | FNV-1a hash of `len` bytes at `s` |
| `hash.fnv1a_str(s)` | FNV-1a hash of string `s` |
| `hash.djb2(s, len)` | djb2 hash of `len` bytes at `s` |
| `hash.djb2_str(s)` | djb2 hash of string `s` |

## Example

```q
bring "std.hash";

craft main() -> void {
    make s: str = "hello";
    make h: u64 = hash.fnv1a_str(s);
    print(`FNV-1a: ${h}`);
    send;
}
```
