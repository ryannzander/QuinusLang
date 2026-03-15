# Move Semantics

The `move` keyword explicitly transfers ownership of a value.

## Syntax

```q
make y = move x;
```

After `move x`, `x` is considered moved and should not be used again in that scope.

## Use Cases

- Transferring ownership of a value to another binding
- Making intent explicit when passing values to functions that take ownership
- Preparing for future ownership/borrow rules

## Example

```q
craft main() -> void {
    make x: i32 = 42;
    make y = move x;
    // x is moved; use y instead
    print(y);
    send;
}
```

## Semantic Checking

The compiler tracks moved variables. Using a variable after it has been moved is an error.
