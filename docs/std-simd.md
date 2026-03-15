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
bring "std.simd";

craft main() -> void {
    hazard {
        cblock { " float buf[4] = {1.0f, 2.0f, 3.0f, 4.0f}; " }
    }
    // Use simd.loadu_ps, add_ps, mul_ps, storeu_ps with hazard/cblock for setup
    send;
}
```

SIMD requires `hazard` blocks for buffer setup. The compiler adds `#include <xmmintrin.h>` when simd is used.
