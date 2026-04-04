---
title: Q++
hide:
  - navigation
  - toc
---

<div class="ql-hero" markdown>
<div class="ql-hero-inner" markdown>

<img src="assets/logo-transparent.png" alt="Q++" class="ql-hero-logo" />

# Q++

**A systems programming language with assembly-level control, readable syntax, and an LLVM-powered native compiler.**

[Get Started](install.md){ .md-button .md-button--primary .ql-hero-btn }
[Quick Tour](tour.md){ .md-button .ql-hero-btn }
[GitHub](https://github.com/ryannzander/Q++){ .md-button .ql-hero-btn }

</div>
</div>

<div class="ql-code-preview" markdown>

```q
craft main() -> void {
    make shift name: str = "world";
    print("Hello, " + name + "!");

    make shift nums: [i32; 5] = { 1, 2, 3, 4, 5 };
    foreach n in nums {
        check (n > 3) {
            print(n);
        }
    }
    send;
}
```

</div>

<div class="ql-features" markdown>

<div class="ql-feature-grid" markdown>

<div class="ql-feature-card" markdown>
### :material-lightning-bolt: LLVM Backend
Compiles to native machine code via LLVM 18. No interpreter, no VM, no garbage collector. Just fast executables.
</div>

<div class="ql-feature-card" markdown>
### :material-shield-check: Safety by Design
Dangerous operations require explicit `hazard` blocks. Pointers use `link`/`mark`/`reach` for visibility. No hidden costs.
</div>

<div class="ql-feature-card" markdown>
### :material-book-open-variant: Readable Syntax
Keywords like `craft`, `send`, `check`, `loopwhile` make low-level code read like intent. No cryptic symbols.
</div>

<div class="ql-feature-card" markdown>
### :material-package-variant: Rich Standard Library
18 stdlib modules: file I/O, networking, math, vectors, maps, string manipulation, SIMD, terminal colors, and more.
</div>

<div class="ql-feature-card" markdown>
### :material-memory: Zero Hidden Runtime
No hidden allocations. Suitable for kernels, bootloaders, firmware, and embedded systems. Inline assembly via `machine` blocks.
</div>

<div class="ql-feature-card" markdown>
### :material-tools: Batteries Included
Package manager, formatter, file watcher, REPL, and syntax highlighting. One `quinus` binary does it all.
</div>

</div>
</div>

<div class="ql-quick-links" markdown>

## Explore

<div class="ql-link-grid" markdown>

- :material-rocket-launch: **[Quick Tour](tour.md)** -- Learn Q++ in 15 minutes
- :material-book: **[Language Reference](language.md)** -- Full syntax and semantics
- :material-library: **[Standard Library](stdlib-index.md)** -- All 18 modules documented
- :material-console: **[CLI Reference](cli.md)** -- Every command explained
- :material-download: **[Installation](install.md)** -- Installer, portable zip, or build from source
- :material-cog: **[LLVM Backend](llvm-backend.md)** -- How the compiler works under the hood

</div>
</div>
