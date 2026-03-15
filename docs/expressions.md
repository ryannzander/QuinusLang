# Expressions

Operators and expression syntax in QuinusLang.

## Arithmetic

| Operator | Meaning | Example |
|----------|---------|---------|
| `+` | Addition | `a + b` |
| `-` | Subtraction | `a - b` |
| `*` | Multiplication | `a * b` |
| `/` | Division | `a / b` |
| `%` | Modulo | `a % b` |

## Comparison

| Operator | Meaning | Example |
|----------|---------|---------|
| `==` | Equal | `a == b` |
| `!=` | Not equal | `a != b` |
| `<` | Less than | `a < b` |
| `<=` | Less or equal | `a <= b` |
| `>` | Greater than | `a > b` |
| `>=` | Greater or equal | `a >= b` |

## Logical

| Operator | Meaning | Example |
|----------|---------|---------|
| `&&` | And | `a && b` |
| `\|\|` | Or | `a \|\| b` |
| `!` | Not | `!flag` |

## Other Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `as` | Cast | `x as usize` |
| `mark` | Address-of | `mark x` |
| `reach` | Dereference | `reach p` |
| `move` | Move | `move x` |

## Literals

```q
42           // Integer
3.14         // Float
true         // Boolean
false        // Boolean
"hello"      // String
`Hello, ${name}!`  // Interpolated string
```

## Compound Expressions

**Array access**: `arr[i]`

**Field access**: `point.x`

**Function call**: `add(1, 2)`

**Tuple**: `(1, "a", true)`

**Range**: `0..10` (exclusive), used in foreach

**Result constructors**: `Ok(value)`, `Err(error)`
