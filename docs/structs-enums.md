# Structs and Enums

## Structs: form

```q
form Point {
    x: i32,
    y: i32
}
```

Access with the dot operator:

```q
make shift p: Point;
p.x = 10;
p.y = 20;
make shift px: i32 = p.x;
```

## Enums: state

```q
state Color {
    Red,
    Green,
    Blue
}
```

Variants are implicitly 0, 1, 2, ...

```q
make shift c: Color = Red;
check (c == Green) { ... }
```

## Unions: fusion

All fields share the same memory:

```q
fusion Maybe {
    value: i32
}
```

## Bitfields

Struct fields can specify a bit width for packed storage:

```q
form StatusReg {
    flags: u32 : 8,
    mode: u32 : 4,
    reserved: u32 : 20
}
```

Use `: N` after the type to allocate N bits. Useful for hardware registers and compact data.

## Enum Payloads

Enums can carry data in variants:

```q
state Option(T) {
    None,
    Some(T)
}

state Result(Ok, Err) {
    Ok(Ok),
    Err(Err)
}
```

Use `choose` to pattern match and extract payloads.

## Struct Methods: impl

Add methods to structs with `impl` blocks:

```q
form Point {
    x: f64,
    y: f64
}

impl Point {
    craft magnitude(self: Point) -> f64 {
        send sqrt(self.x * self.x + self.y * self.y);
    }
}

craft main() -> void {
    make p: Point = ...;
    make m: f64 = p.magnitude();
    send;
}
```

## Nested Types

```q
form Vec2 { x: f64, y: f64 }

form Entity {
    pos: Vec2,
    vel: Vec2,
    id: i32
}
```
