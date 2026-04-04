# Contributing to Q++

Thank you for your interest in contributing to Q++!

## Building

### Prerequisites

- **Rust** — Install from [rustup.rs](https://rustup.rs)
- **C compiler** — GCC, Clang, or MSVC (for compiling emitted C code)

### Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

The compiler binary will be at `target/release/qpp` (or `qpp.exe` on Windows).

## Testing

```bash
cargo test
```

## Development Workflow

1. **Fork** the repository
2. **Create a branch** — `feature/your-feature` or `fix/your-fix`
3. **Make changes** — Follow existing code style
4. **Run tests** — `cargo test`
5. **Submit a PR** — Describe your changes clearly

## Commit Conventions

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation only
- `test:` — Adding or updating tests
- `refactor:` — Code change that neither fixes a bug nor adds a feature
- `chore:` — Maintenance tasks

Example: `feat(parser): add extern declarations`

## Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`

## Project Structure

- `src/` — Compiler source (lexer, parser, semantic, codegen)
- `stdlib/` — Standard library modules (fs, os, io)
- `docs/` — Documentation
- `tests/` — Integration tests
