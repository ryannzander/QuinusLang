# std.math

Math operations.

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `abs_i32` | `(x: i32) -> i32` | Absolute value (integer). |
| `abs_f64` | `(x: f64) -> f64` | Absolute value (float). |
| `min_i32` | `(a: i32, b: i32) -> i32` | Minimum of two integers. |
| `max_i32` | `(a: i32, b: i32) -> i32` | Maximum of two integers. |
| `min_f64` | `(a: f64, b: f64) -> f64` | Minimum of two floats. |
| `max_f64` | `(a: f64, b: f64) -> f64` | Maximum of two floats. |
| `sqrt_f64` | `(x: f64) -> f64` | Square root. |
| `add_checked_i32` | `(a: i32, b: i32) -> Result(i32, i32)` | Checked addition; `Err(0)` on overflow. |
| `sub_checked_i32` | `(a: i32, b: i32) -> Result(i32, i32)` | Checked subtraction; `Err(0)` on overflow. |
| `mul_checked_i32` | `(a: i32, b: i32) -> Result(i32, i32)` | Checked multiplication; `Err(0)` on overflow. |

## Example

```q
bring "std.math";

craft main() -> void {
    make x: f64 = math.sqrt_f64(16.0);
    make m: i32 = math.min_i32(1, 2);
    send;
}
```
