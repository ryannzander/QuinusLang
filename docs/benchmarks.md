# Q++ Benchmarks

Cross-language benchmarks comparing Q++ against C, Rust, and Zig.

## How to Run

From the project root:

```powershell
.\benchmarks\run.ps1
```

With options:

```powershell
.\benchmarks\run.ps1 -Runs 10 -Release
```

- `-Runs N` — Number of runs per benchmark (default: 5)
- `-Release` — Use release/optimized builds

## Prerequisites

1. **Q++**: `cargo build --release` (compiler at `target/release/qpp.exe`)
2. **C**: gcc or MSVC (`winget install mingw` or Visual Studio Build Tools)
3. **Rust**: `rustc` (from rustup)
4. **Zig**: `zig` (from ziglang.org or `winget install zig`)

## Benchmarks

| Benchmark | Description |
|-----------|-------------|
| sum | Simple loop summing 1..100_000_000 |
| fib | Iterative Fibonacci(40) |
| sieve | Sieve of Eratosthenes, primes up to 1,000,000 |
| mandelbrot | Mandelbrot set 160x120 grid, 100 iterations max |
| nbody | 5-body simulation, 50,000 steps |

## Results

*(Run `.\benchmarks\run.ps1` to populate)*

| Benchmark | Quinus | C | Rust | Zig |
|-----------|--------|---|------|-----|
| sum | - | - | - | - |
| fib | - | - | - | - |
| sieve | - | - | - | - |
| mandelbrot | - | - | - | - |
| nbody | - | - | - | - |

Times are in seconds (mean ± stddev over N runs).
