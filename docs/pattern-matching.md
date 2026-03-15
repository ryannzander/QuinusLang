# Pattern Matching: choose

The `choose` expression performs pattern matching on enums, tuples, and values.

## Basic Syntax

```q
choose expr {
    Pattern1 => { statements }
    Pattern2 => { statements }
}
```

## Matching Enums

```q
state Option(T) {
    None,
    Some(T)
}

choose maybe_value {
    Some(x) => {
        print(x);
    }
    None => {
        print("No value");
    }
}
```

## Matching Result

```q
choose parse_input() {
    Ok(value) => {
        use(value);
    }
    Err(msg) => {
        print(`Failed: ${msg}`);
    }
}
```

## Matching Tuples

```q
make (a, b) = get_pair();
choose (a, b) {
    (0, 0) => { print("origin"); }
    (x, 0) => { print(`on x-axis: ${x}`); }
    (0, y) => { print(`on y-axis: ${y}`); }
    (x, y) => { print(`point: ${x}, ${y}`); }
}
```

## Matching with Bindings

Patterns can bind variables for use in the arm body:

```q
choose result {
    Ok(val) => { make x = val; ... }   // val bound
    Err(e) => { make err = e; ... }   // e bound
}
```

## Exhaustiveness

For enums, the compiler checks that all variants are covered. Missing a variant is a compile error.
