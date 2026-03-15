# Pointers and Memory

## Pointer Type: link

```q
make shift p: link i32;
make shift s: link str;
```

## Address-of: mark

```q
make shift x: i32 = 42;
make shift p: link i32 = mark x;
```

## Dereference: reach

```q
make shift value: i32 = reach p;   // Read
reach p = 99;                       // Write
```

## Example

```q
craft main() -> void {
    make shift x: i32 = 42;
    make shift p: link i32 = mark x;
    print(reach p);   // 42
    reach p = 100;
    print(x);         // 100
    send;
}
```

## Null Pointers

Use `0` or a cast when interfacing with C. Dereferencing null is undefined behavior.

## C Mapping

| QuinusLang | C |
|------------|---|
| `link T` | `T*` |
| `mark x` | `&x` |
| `reach p` | `*p` |
