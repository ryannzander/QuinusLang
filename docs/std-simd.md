# std.simd

SIMD intrinsics (SSE). Wraps `xmmintrin.h`.

## Functions

| Function | Description |
|----------|-------------|
| `loadu_ps(p)` | Load 4 floats from unaligned pointer |
| `storeu_ps(p, a)` | Store 4 floats to unaligned pointer |
| `add_ps(a, b)` | Add 4 floats |
| `mul_ps(a, b)` | Multiply 4 floats |

## Example

```q
bring "simd";

craft main() -> void {
    hazard {
        cblock { " float buf[4] = {1.0f, 2.0f, 3.0f, 4.0f}; float out[4]; " }
    }
    make p: link f32 = 0;
    make a: link void = simd.loadu_ps(p);
    send;
}
```
