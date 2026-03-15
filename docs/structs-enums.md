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

## Nested Types

```q
form Vec2 { x: f64, y: f64 }

form Entity {
    pos: Vec2,
    vel: Vec2,
    id: i32
}
```
