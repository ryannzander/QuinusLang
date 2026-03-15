# Modules and FFI

## Modules: realm

```q
realm math {
    craft add(a: i32, b: i32) -> i32 {
        send a + b;
    }
}
```

## Imports: bring

```q
bring "std.fs";
bring "std.os";

craft main() -> void {
    make f = fs.open_file("x.txt", "r");
    os.run("echo hello");
    send;
}
```

## C FFI: extern craft

Call C library functions:

```q
extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;

craft main() -> void {
    make f = fopen("data.txt", "r");
    check (f != 0) {
        fclose(f);
    }
    send;
}
```

## Passing Structs

Structs are passed by value by default. Use `link T` for pointers when interfacing with C.
