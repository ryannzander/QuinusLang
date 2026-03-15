# Embedded Development

QuinusLang targets systems programming: kernels, bootloaders, firmware, embedded.

## Zero Hidden Runtime

- No mandatory allocator
- No hidden initialization
- Compiles to C, then to native

## Considerations

### Custom Allocators

Use `malloc`/`free` via FFI, or implement your own allocator. No built-in heap required.

### Compile Flags

Use conditional compilation (when available) for target-specific code:

```q
#if TARGET == "arm"
    // ARM-specific
#endif
```

### Bitfields

For hardware registers, use structs with explicit bit width:

```q
form StatusReg {
    flags: u32 : 8,
    mode: u32 : 4,
    reserved: u32 : 20,
}
```

### Minimal Stdlib

Import only what you need. `bring "std.fs"` pulls in file I/O; omit it for bare-metal.
