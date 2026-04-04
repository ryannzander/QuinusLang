# Compile Flags

Module-level conditional compilation with `#define`, `#if`, `#ifdef`, `#ifndef`, `#else`, `#endif`.

## Directives

| Directive | Description |
|-----------|-------------|
| `#define NAME` | Define a symbol for `#if` / `#ifdef` |
| `#if SYMBOL` | Include block if symbol is defined or equals `1` / `true` |
| `#ifdef SYMBOL` | Include block if symbol is defined |
| `#ifndef SYMBOL` | Include block if symbol is not defined |
| `#else` | Alternate block when condition is false |
| `#endif` | End conditional block |

## Example

```q
#define DEBUG

#if DEBUG
craft main() -> void {
    make x: i32 = 1;
    send;
}
#else
craft main() -> void { send; }
#endif
```

## Command-line defines

Pass symbols via `--define` when building:

```bash
qpp build --define DEBUG
qpp build --define RELEASE --define TRACE
```
