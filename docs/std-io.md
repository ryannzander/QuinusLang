# std.io

Output helpers.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `println` | `(...) -> void` | Print to stdout with newline. |

## Example

```q
bring "std.io";

craft main() -> void {
    io.println("Hello");
    send;
}
```
