# std.fs

File system operations.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `open_file` | `(path: str, mode: str) -> link void` | Open file. Returns null on failure. |
| `close` | `(stream: link void) -> i32` | Close file. |
| `read_all` | `(stream: link void) -> str` | Read entire file into string. |
| `exists` | `(path: str) -> bool` | Check if file exists. |
| `write_all` | `(path: str, content: str) -> i32` | Write string to file. Returns 1 on success. |

## Example

```q
bring "std.fs";

craft main() -> void {
    make f = fs.open_file("data.txt", "r");
    check (f != 0) {
        make content = fs.read_all(f);
        fs.close(f);
        print(content);
    }
    send;
}
```
