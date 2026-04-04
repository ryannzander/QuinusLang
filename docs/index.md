---
title: Q++
hide:
  - navigation
  - toc
---

<!-- TypeScript-inspired landing: full-width blue hero, two-column grid, editor mockup -->

<div class="ql-ts-home">

<section class="ql-ts-hero">
<div class="ql-ts-hero-inner">
<div class="ql-ts-hero-grid">

<div class="ql-ts-copy">

<img src="assets/logo-brand.svg" alt="" class="ql-ts-logo" width="120" height="120" />

<h1 class="ql-ts-headline">Q++ is <strong>systems code</strong> with <strong>readable</strong> syntax.</h1>

<p class="ql-ts-lead">A systems programming language with assembly-level control, an LLVM-powered native compiler, and no hidden runtime. Tooling at any scale.</p>

<p class="ql-ts-cta-wrap"><a href="install/" class="ql-ts-cta"><span class="ql-ts-cta-main">Get started</span><span class="ql-ts-cta-sub">Installer, portable zip, or build from source</span></a></p>

<p class="ql-ts-hero-actions">
<a href="tour.md" class="ql-ts-link">Quick tour</a><span class="ql-ts-dot" aria-hidden="true"> · </span><a href="https://github.com/ryannzander/QuinusLang" class="ql-ts-link">GitHub</a>
</p>

</div>

<div class="ql-ts-demo">

<div class="ql-ts-win">
<div class="ql-ts-win-tabs" role="tablist">
<span class="ql-ts-tab ql-ts-tab--active">Type checks</span>
<span class="ql-ts-tab">LLVM</span>
<span class="ql-ts-tab">Unsafe</span>
<span class="ql-ts-tab">FFI</span>
</div>
<div class="ql-ts-win-body" markdown="1">

```q
craft main() -> void {
    make shift user: User = {
        name: "Ada",
        role: Role.Admin,
    };
    print(user.display_name);
    send;
}
```

<div class="ql-ts-squiggle" aria-hidden="true"></div>
<div class="ql-ts-errbox">
<span class="ql-ts-err-title">check failed</span>
<code class="ql-ts-err-msg">no field <span class="hl">display_name</span> on <span class="hl">User</span> — did you mean <span class="hl">name</span>?</code>
</div>

</div>
</div>

</div>

</div>
</div>
</section>

<div class="ql-ts-announce">
<a href="https://github.com/ryannzander/QuinusLang/releases" class="ql-ts-announce-link">Latest release</a> — Q++ ships with <code>qpp</code>, formatter, package manager, and REPL.
</div>

</div>

<div class="ql-ts-below" markdown>

<div class="ql-features" markdown>

<div class="ql-feature-grid" markdown>

<div class="ql-feature-card" markdown>
### LLVM backend
Compiles to native machine code via LLVM. No interpreter, no VM, no garbage collector.
</div>

<div class="ql-feature-card" markdown>
### Safety by design
Dangerous operations require explicit `hazard` blocks. Pointers use `link` / `mark` / `reach`.
</div>

<div class="ql-feature-card" markdown>
### Readable syntax
Keywords like `craft`, `send`, `check`, `loopwhile` keep low-level code legible.
</div>

<div class="ql-feature-card" markdown>
### Standard library
File I/O, networking, math, SIMD, terminal colors, and more in one toolchain.
</div>

<div class="ql-feature-card" markdown>
### Zero hidden runtime
Suitable for kernels, bootloaders, firmware, and embedded. Inline `machine` blocks.
</div>

<div class="ql-feature-card" markdown>
### Batteries included
Formatter, watcher, syntax highlighting — one `qpp` binary does it all.
</div>

</div>
</div>

<div class="ql-quick-links" markdown>

## Explore

<div class="ql-link-grid" markdown>

- :material-rocket-launch: **[Quick Tour](tour.md)** — Learn Q++ in minutes
- :material-book: **[Language Reference](language.md)** — Syntax and semantics
- :material-library: **[Standard Library](stdlib-index.md)** — All modules
- :material-console: **[CLI Reference](cli.md)** — Commands and flags
- :material-download: **[Installation](install.md)** — Setup options
- :material-cog: **[LLVM Backend](llvm-backend.md)** — How compilation works

</div>
</div>

</div>
