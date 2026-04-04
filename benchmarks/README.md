# Q++ Benchmark Suite

Cross-language benchmarks comparing Q++ against C, Rust, and Zig.

## Benchmarks

| Benchmark | Description |
|-----------|--------------|
| **sum** | Simple loop summing 1..N (baseline) |
| **fib** | Fibonacci(40) - iterative or recursive |
| **sieve** | Sieve of Eratosthenes, primes up to N |
| **mandelbrot** | Mandelbrot set, small grid iteration count |
| **nbody** | N-body simulation, ~50k iterations |

## Prerequisites

- **Q++**: Build the compiler first: `cargo build --release` (or `cargo build` for debug)
- **C**: MSVC or MinGW (e.g. `winget install mingw`)
- **Rust**: `rustc` (from rustup)
- **Zig**: `zig` (from ziglang.org or winget)

## How to Run

From the project root:

```powershell
.\benchmarks\run.ps1
```

Or with custom run count:

```powershell
.\benchmarks\run.ps1 -Runs 10
```

The script will:
1. Build each benchmark in all four languages
2. Run each executable N times (default: 5)
3. Report mean and standard deviation in a table

## Results Summary

See [docs/benchmarks.md](../docs/benchmarks.md) for results table and instructions.
