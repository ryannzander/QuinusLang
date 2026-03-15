# std.path

Path manipulation utilities.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `join` | `(a: str, b: str) -> str` | Join two path segments with `/`. |

## Example

```q
bring "path";

craft main() -> void {
    make p: str = path.join("usr", "local");
    make full: str = path.join(p, "bin");
    print(full);
    send;
}
```
