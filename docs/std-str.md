# std.str

String utilities.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `trim` | `(s: str) -> str` | Trim whitespace from both ends. |
| `concat` | `(a: str, b: str) -> str` | Concatenate two strings. |

## Example

```q
bring "std.str";

craft main() -> void {
    make s: str = str.trim("  hello  ");
    make t: str = str.concat("a", "b");
    send;
}
```
