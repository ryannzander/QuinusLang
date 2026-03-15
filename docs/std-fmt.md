# std.fmt

String formatting (sprintf-style). Uses C printf format specifiers (`%d`, `%s`, etc.).

## Buffer-Based (write to existing buffer)

| Function | Signature | Description |
|----------|-----------|-------------|
| `sprintf_i` | `(buf: str, size: usize, fmt: str, a: i64) -> i32` | Format integer into buffer. |
| `sprintf_s` | `(buf: str, size: usize, fmt: str, s: str) -> i32` | Format string into buffer. |
| `sprintf_ii` | `(buf, size, fmt, a: i64, b: i64) -> i32` | Format two integers. |
| `sprintf_si` | `(buf, size, fmt, s: str, a: i64) -> i32` | Format string and integer. |
| `sprintf_ss` | `(buf, size, fmt, a: str, b: str) -> i32` | Format two strings. |

## Allocating (returns new string)

| Function | Signature | Description |
|----------|-----------|-------------|
| `alloc_i` | `(fmt: str, a: i64) -> str` | Format integer, allocate result. |
| `alloc_s` | `(fmt: str, s: str) -> str` | Format string, allocate result. |
| `alloc_si` | `(fmt: str, s: str, a: i64) -> str` | Format string and integer. |

## Example

```q
bring "std.fmt";

craft main() -> void {
    make s: str = fmt.alloc_i("%d", 42);
    print(s);
    send;
}
```
