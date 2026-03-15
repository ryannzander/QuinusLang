# String Interpolation

Backtick strings allow embedding expressions with `${expr}`.

## Syntax

```q
`Hello, ${name}!`
`Result: ${x + y}`
`Path: ${base}/${file}`
```

## Usage

String interpolation works with `print`, `write`, and `writeln`:

```q
craft main() -> void {
    make name: str = "World";
    print(`Hello, ${name}!`);
    make x: i32 = 42;
    writeln(`The answer is ${x}`);
    send;
}
```

## Escaping

Inside backtick strings:

- `${expr}` — embed expression
- Use `\` for escape sequences if needed
- Nested quotes: `"nested"` inside the expression

## Example

```q
craft greet(name: str, count: i32) -> void {
    print(`Hello ${name}, you have ${count} items.`);
    send;
}
```
