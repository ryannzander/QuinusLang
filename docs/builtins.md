# Built-in Functions

QuinusLang provides built-in functions without importing any module.

## Output

| Function | Signature | Description |
|----------|-----------|-------------|
| `print` | `(args...) -> void` | Print to stdout (printf-style) |
| `write` | `(args...) -> void` | Print without newline |
| `writeln` | `() -> void` | Print newline only |

Use string interpolation with `print` and `write`: `print(\`x = ${x}\`);`

## Input

| Function | Signature | Description |
|----------|-----------|-------------|
| `read` | `() -> i32` | Read integer from stdin (scanf) |

## Length

| Function | Signature | Description |
|----------|-----------|-------------|
| `len` | `(arr: [T; N]) -> usize` | Array length |
| `strlen` | `(s: str) -> usize` | String length |

## Abort

| Function | Signature | Description |
|----------|-----------|-------------|
| `panic` | `() -> void` | Abort program |
| `assert` | `(cond: bool) -> void` | Abort if condition is false |
| `assert` | `(cond: bool, msg: str) -> void` | Abort with custom message if condition is false |

## Example

```q
craft main() -> void {
    print(42);
    writeln();
    make x: i32 = read();
    assert(x >= 0);
    assert(x < 100, "x must be less than 100");
    print(`You entered ${x}`);
    send;
}
```
