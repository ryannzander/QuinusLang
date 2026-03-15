# std.os

Process execution and environment.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `run` | `(cmd: str) -> i32` | Execute shell command. Returns exit code. |
| `getenv` | `(name: str) -> str` | Get environment variable. Returns empty string if not set. |
| `cwd` | `() -> str` | Get current working directory. |

## Example

```q
bring "std.os";

craft main() -> void {
    make code: i32 = os.run("echo hello");
    make home: str = os.getenv("HOME");
    make dir: str = os.cwd();
    send;
}
```
