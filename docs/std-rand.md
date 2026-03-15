# std.rand

Pseudorandom number generation (uses C rand/srand).

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `next` | `() -> i32` | Next random integer (0 to RAND_MAX). |
| `seed` | `(s: u32) -> void` | Seed the generator. |

## Example

```q
bring "rand";

craft main() -> void {
    rand.seed(42);
    make r: i32 = rand.next();
    print(r);
    send;
}
```
