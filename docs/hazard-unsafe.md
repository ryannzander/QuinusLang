# Hazard and Unsafe

Operations that bypass safety must be inside a `hazard` block.

## Hazard Blocks

```q
hazard {
    reach raw_ptr = value;
    // Unsafe code
}
```

This makes it explicit that the code may have undefined behavior if used incorrectly.

## Inline Assembly: machine

Emit raw assembly inside `hazard`:

```q
hazard {
    machine { "mov eax, 1" };
    machine { "nop" };
}
```

Each string is one instruction. Syntax is GCC-style (`__asm__ __volatile__`).

## Inline C: cblock

Emit raw C code inside `hazard`:

```q
hazard {
    cblock { " int x = 0; " };
    cblock { " printf(\"x = %d\\n\", x); " };
}
```

Use for low-level C interop, SIMD setup, or when you need exact control over emitted code.

## Platform Support

- **GCC / Clang**: Full inline assembly support
- **MSVC x64**: Inline assembly not supported; instruction is skipped

Use MinGW or Clang on Windows for assembly.

## Use Cases

- Direct hardware access (memory-mapped I/O)
- CPU-specific instructions (SIMD, crypto)
- Kernel and bootloader code
- Performance-critical inner loops

## Safety

No runtime checks in hazard blocks. Document constraints and invariants.
