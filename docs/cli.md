# CLI Reference

## Build and Run

| Command | Description |
|---------|-------------|
| `quinus build [path]` | Compile to executable (default: `.`) |
| `quinus build --release` | Optimized build |
| `quinus build --emit-c` | Emit C only, do not compile |
| `quinus build --define NAME` | Define for `#if` / `#ifdef` (repeatable) |
| `quinus run [path]` | Build and run |

## Package Management

| Command | Description |
|---------|-------------|
| `quinus init [path]` | Create new package |
| `quinus add NAME [--git URL]` | Add dependency |
| `quinus remove NAME` | Remove dependency |
| `quinus update` | Update dependencies |
| `quinus publish` | Publish to registry (validate and tag) |

## Development

| Command | Description |
|---------|-------------|
| `quinus fmt [path]` | Format source files |
| `quinus watch [path]` | Rebuild on file changes |
| `quinus check [path]` | Parse and type-check only (no codegen) |
| `quinus repl` | Interactive REPL (parse and show AST) |
| `quinus lsp` | Language Server Protocol (IDE support) |

## Debug

| Command | Description |
|---------|-------------|
| `quinus parse PATH` | Parse file and show AST |
| `quinus preprocess PATH` | Resolve imports, output flattened source |
