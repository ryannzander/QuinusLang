# With Blocks (Scope-Bound Resources)

`with` provides scope-bound resources that are automatically cleaned up when the block exits.

## Syntax

```q
with var = expr { body }
```

The variable is bound to the result of `expr` and is valid only inside the block. When the block exits (normally or via early return), the resource is automatically closed.

## Supported Resources

Currently, `fs.open_file` is recognized: the file handle is closed with `fs.close` when the block exits.

```q
bring "std.fs";

craft main() -> void {
    with f = fs.open_file("x.txt", "r") {
        make content = fs.read_all(f);
        print(content);
    }
    // f is closed here; f is invalid after
    send;
}
```

## Comparison with defer

| `defer` | `with` |
|---------|--------|
| Explicit cleanup block | Automatic cleanup for known resources |
| `defer { fs.close(f); }` | `with f = fs.open_file(...) { ... }` |
| Variable in outer scope | Variable scoped to block only |

`with` combines scoping and cleanup: the resource is only in scope inside the block, and cleanup is automatic for supported resource types.
