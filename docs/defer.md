# Defer

`defer` runs a block of code when the current scope exits, whether normally or via early return.

## Syntax

```q
defer {
    // cleanup code
}
```

## Use Cases

**Resource cleanup**:

```q
craft process_file(path: str) -> void {
    make f = fs.open_file(path, "r");
    defer { fs.close(f); }
    make content = fs.read_all(f);
    print(content);
    send;
}
```

**Unlock mutex** (when sync primitives exist):

```q
defer { mutex.unlock(); }
// ... critical section ...
```

**Temporary state**:

```q
defer { restore_state(); }
set_temp_state();
// ... use temp state ...
```

## Execution Order

Multiple `defer` blocks in the same scope run in **reverse** order (last deferred runs first):

```q
defer { print("first"); }
defer { print("second"); }
// When scope exits: "second" then "first"
```
