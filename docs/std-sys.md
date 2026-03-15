# std.sys

Platform detection (compile-time).

## Functions

| Function | Description |
|----------|-------------|
| `sys.is_windows()` | Returns 1 on Windows, 0 otherwise |
| `sys.is_unix()` | Returns 1 on Unix/Linux/macOS, 0 on Windows |

## Example

```q
bring "std.sys";

craft main() -> void {
    check (sys.is_windows() == 1) {
        print("Running on Windows");
    }
    otherwise {
        print("Running on Unix");
    }
    send;
}
```
