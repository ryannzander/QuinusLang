# QuinusLang

A systems programming language with assembly-level control and readable syntax. Compiles to C, then to native executables.

## Quick Start

```bash
quinus init
quinus run
```

Your entry point is `src/main.q`:

```q
craft main() -> void {
    print(42);
    send;
}
```

## Installation

```bash
cargo install --path .
```

## Links

- [Language Reference](language.md)
- [CLI Reference](cli.md)
- [Contributing](../CONTRIBUTING.md)
- [GitHub](https://github.com/ryannzander/QuinusLang)
