# Language Reference

See the [README](../README.md#language-reference) for full syntax.

## Key concepts

- **Variables**: `make` (immutable), `make shift` (mutable)
- **Functions**: `craft name(args) -> return_type { ... }`
- **Control**: `check`/`otherwise`, `loopwhile`, `foreach`, `for`
- **Types**: `i32`, `u64`, `str`, `link T`, `[T; N]`
- **Tuple destructuring**: `make (a, b) = fn_returns_tuple();`
- **Cast**: `expr as type` (e.g. `x as usize`)
- **C FFI**: `extern craft name(args) -> ret;`

## Standard Library

- **fs** — `open_file`, `close`, `read_all`, `exists`, `write_all`
- **os** — `run(cmd)` for process execution
- **io** — `println`
