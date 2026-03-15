# Result Type

The `Result(OkType, ErrType)` type represents a value that is either success (`Ok`) or failure (`Err`).

## Syntax

```q
state Result(Ok, Err) {
    Ok(Ok),
    Err(Err)
}
```

Use `Result(i32, str)` for a function that returns either an integer or an error string.

## Creating Results

```q
craft parse(s: str) -> Result(i32, str) {
    check (s == 0) {
        send Err("null string");
    }
    // ... parse logic ...
    send Ok(42);
}
```

## Pattern Matching with choose

```q
choose parse(input) {
    Ok(value) => {
        print(value);
    }
    Err(msg) => {
        print(`Error: ${msg}`);
    }
}
```

## Checked Arithmetic

The math module provides checked operations that return `Result` on overflow:

```q
bring "std.math";

craft main() -> void {
    choose math.add_checked_i32(2_000_000_000, 2_000_000_000) {
        Ok(sum) => print(sum);
        Err(_) => print("Overflow!");
    }
    send;
}
```

## Built-in Checked Functions

| Function | Returns | On Overflow |
|----------|---------|-------------|
| `math.add_checked_i32` | `Result(i32, i32)` | `Err(0)` |
| `math.sub_checked_i32` | `Result(i32, i32)` | `Err(0)` |
| `math.mul_checked_i32` | `Result(i32, i32)` | `Err(0)` |
