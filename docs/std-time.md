# std.time

Time operations.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `now` | `() -> i64` | Seconds since Unix epoch. |

## Example

```q
bring "std.time";

craft main() -> void {
    make t: i64 = time.now();
    print(t);
    send;
}
```
