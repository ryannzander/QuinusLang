# Control Flow

## Conditionals: check and otherwise

Use `check` for if and `otherwise` for else:

```q
check (x > 0) {
    print("positive");
}
otherwise {
    print("non-positive");
}
```

The condition must be a boolean expression in parentheses.

## While Loop: loopwhile

```q
make shift i: i32 = 0;
loopwhile (i < 10) {
    print(i);
    i = i + 1;
}
```

## For Loop

C-style for loop with init, condition, and step:

```q
for (make shift i: i32 = 0; i < 10; i = i + 1) {
    print(i);
}
```

## For-Each Loop: foreach

```q
foreach item in arr {
    print(item);
}
```

`item` is a new variable in each iteration.

## Break and Continue

- `stop` — break out of loop
- `skip` — continue to next iteration

```q
loopwhile (true) {
    check (done) { stop; }
    check (skip_this) { skip; }
}
```

## Comparison Operators

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<`, `<=` | Less than, less or equal |
| `>`, `>=` | Greater than, greater or equal |
