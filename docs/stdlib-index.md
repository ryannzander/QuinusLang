# Standard Library

| Module | Description |
|--------|-------------|
| [fs](std-fs.md) | File I/O: open_file, close, read_all, exists, write_all |
| [os](std-os.md) | Process execution: run |
| [io](std-io.md) | Output: println |
| [math](std-math.md) | Math: abs, sqrt, fmin, fmax |
| [str](std-str.md) | String helpers: trim, concat |
| [vec](std-vec.md) | Dynamic vectors |
| [map](std-map.md) | Map/dict |
| [fmt](std-fmt.md) | String formatting |
| [time](std-time.md) | Time: now |
| [rand](std-rand.md) | Random: next, seed |
| [arena](std-arena.md) | Allocator: alloc, dealloc |
| [simd](std-simd.md) | SIMD: loadu_ps, storeu_ps, add_ps, mul_ps |

## Usage

```q
bring "std.fs";
bring "std.math";

craft main() -> void {
    make f = fs.open_file("x.txt", "r");
    make x: f64 = math.sqrt(16.0);
    send;
}
```
