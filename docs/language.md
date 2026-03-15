# Language Reference

## Key concepts

- **Variables**: `make` (immutable), `make shift` (mutable)
- **Functions**: `craft name(args) -> return_type { ... }`
- **Control**: `check`/`otherwise`, `loopwhile`, `foreach`, `for`
- **Types**: `i32`, `u64`, `str`, `link T`, `[T; N]`
- **Tuple destructuring**: `make (a, b) = fn_returns_tuple();`
- **Cast**: `expr as type` (e.g. `x as usize`)
- **String interpolation**: `` `Hello, ${name}!` `` — backtick strings with `${expr}`
- **C FFI**: `extern craft name(args) -> ret;`

## Example

```q
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}

craft main() -> void {
    make x: i32 = add(1, 2);
    print(`Result: ${x}`);
    send;
}
```

## Standard Library

- **fs** — `open_file`, `close`, `read_all`, `exists`, `write_all`
- **os** — `run(cmd)` for process execution
- **io** — `println`
