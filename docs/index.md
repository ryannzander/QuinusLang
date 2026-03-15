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

## Next Steps

- [Installation](install.md) — Installer, portable zip, or from source
- [Quick Tour](tour.md) — 15-minute walkthrough
- [Language Reference](language.md) — Full syntax
- [Standard Library](stdlib-index.md) — Available modules
- [CLI Reference](cli.md) — All commands

## Links

- [GitHub](https://github.com/ryannzander/QuinusLang)
