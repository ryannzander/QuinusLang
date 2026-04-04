# Quick Tour

A 15–20 minute walkthrough of Q++.

## 1. Create a Project

```bash
qpp init my-app
cd my-app
```

This creates `src/main.q` and `qpp.toml`.

## 2. Variables

```q
make x: i32 = 42;        // Immutable
make shift y: i32 = 10;   // Mutable
y = y + 1;
```

- `make` — immutable binding
- `make shift` — mutable variable

## 3. Functions

```q
craft add(a: i32, b: i32) -> i32 {
    send a + b;
}

craft main() -> void {
    print(add(1, 2));
    send;
}
```

- `craft` — function
- `send` — return

## 4. Control Flow

```q
check (x > 0) {
    print("positive");
}
otherwise {
    print("non-positive");
}

loopwhile (i < 10) {
    i = i + 1;
}

foreach item in arr {
    print(item);
}
```

- `check` / `otherwise` — if / else
- `loopwhile` — while loop
- `foreach` — iterate over array

## 5. Structs

```q
form Point {
    x: i32,
    y: i32
}

craft main() -> void {
    make shift p: Point = ...;
    p.x = 10;
    p.y = 20;
    send;
}
```

## 6. Modules

```q
bring "std.fs";

craft main() -> void {
    make f = fs.open_file("data.txt", "r");
    check (f != 0) {
        make content = fs.read_all(f);
        fs.close(f);
        print(content);
    }
    send;
}
```

## 7. Build and Run

```bash
qpp run
# or
qpp build
.\build\output.exe
```

## Next Steps

- [Language Reference](language.md) — Full syntax
- [Standard Library](stdlib-index.md) — Available modules
- [CLI Reference](cli.md) — All commands
