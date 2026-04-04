# CLI Reference

## Build and Run

| Command | Description |
|---------|-------------|
| `qpp build [path]` | Compile to executable (default: `.`) |
| `qpp build --release` | Optimized build |
| `qpp build --emit-llvm` | Emit LLVM IR only, do not compile |
| `qpp build --define NAME` | Define for `#if` / `#ifdef` (repeatable) |
| `qpp run [path]` | Build and run |

## Package Management

| Command | Description |
|---------|-------------|
| `qpp init [path]` | Create new package |
| `qpp add NAME [--git URL]` | Add dependency |
| `qpp remove NAME` | Remove dependency |
| `qpp update` | Update dependencies |
| `qpp publish` | Publish to registry (validate and tag) |

## Development

| Command | Description |
|---------|-------------|
| `qpp fmt [path]` | Format source files |
| `qpp watch [path]` | Rebuild on file changes |
| `qpp check [path]` | Parse and type-check only (no codegen) |
| `qpp repl` | Interactive REPL (parse and show AST) |
| `qpp lsp` | Language Server Protocol (IDE support) |

## Debug

| Command | Description |
|---------|-------------|
| `qpp parse PATH` | Parse file and show AST |
| `qpp preprocess PATH` | Resolve imports, output flattened source |
