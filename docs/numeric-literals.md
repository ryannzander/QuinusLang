# Numeric Literals

## Digit Separators

Use `_` to separate digits in numeric literals for readability:

```q
make million: i32 = 1_000_000;
make bytes: i64 = 1_234_567_890;
make count: u32 = 999_999;
```

Underscores are ignored by the compiler. They do not affect the value.

## Integer Literals

```q
42
-17
0
1_000_000
```

## Float Literals

```q
3.14
1.5e10
2.5e-3
1_000.5
```

## Use Cases

- Large constants: `1_000_000` instead of `1000000`
- Grouping: `999_999` for readability
