# std.fmt

String formatting (sprintf-style).

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `sprintf_i` | `(fmt: str, a: i64) -> str` | Format integer. |
| `sprintf_s` | `(fmt: str, s: str) -> str` | Format string. |
| `sprintf_si` | `(fmt: str, s: str, a: i64) -> str` | Format string and integer. |

Use `%d`, `%s`, etc. in format strings (C printf style).
