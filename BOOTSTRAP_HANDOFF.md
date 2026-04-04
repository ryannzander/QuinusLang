# Q++ Bootstrap Handoff

Handoff document for continuing the bootstrap compiler work from a different device.

---

## What We Did

### Completed (Steps 1–10)

1. **stdlib/vec.q** – Growable arrays: `vec.ptr_new`, `vec.ptr_push`, `vec.ptr_get`, `vec.ptr_len`, `vec.ptr_clear`, `vec.ptr_free`, `vec.u8_*`
2. **stdlib/map.q** – Linear-search map: `map.str_ptr_new`, `map.str_ptr_put`, `map.str_ptr_get`, `map.str_ptr_has`, `map.str_ptr_len`, `map.str_ptr_free`
3. **stdlib/fmt.q** – String formatting: `fmt.sprintf_i`, `fmt.sprintf_s`, `fmt.alloc_i`, `fmt.alloc_s`, etc.
4. **compiler/tokens.q** – Token type constants (INT, IDENT, PLUS, etc.)
5. **compiler/lexer.q** – Hand-written lexer: `lexer.tokenize(source)`, `lexer.token_ty`, `lexer.token_str`, `lexer.token_int`
6. **compiler/ast.q** – AST types: `Expr`, `Stmt`, `Param`, `FnDef`, `Program` (tagged union form)
7. **compiler/parser.q** – Minimal recursive-descent parser: `parser.parse(source)` – parses INT and IDENT only
8. **compiler/semantic.q** – Type-check AST: `semantic.check_expr`, `semantic.symtab_put`, `semantic.symtab_get`
9. **compiler/codegen.q** – Emit C from AST: `codegen.emit_expr`, `codegen.emit_program` (Literal, Ident, Binary)
10. **compiler/main.q** – Driver: reads `input.q`, runs pipeline, writes `build/output.c`

### Rust Compiler Changes

- **src/codegen/c.rs**: Added AST getters (`ql_ast_expr_tag`, `ql_ast_expr_int`, `ql_ast_expr_str`, `ql_ast_expr_left`, `ql_ast_expr_right`), string literal escaping for `\n`/`\r`/`\t`
- **src/main.rs**: Fixed `base_dir` for file builds – when building `compiler/foo.q`, project root is found by walking up from the file’s directory so `stdlib/` imports resolve

### Test Files

- `compiler/lexer_test.q` – Tests lexer
- `compiler/parser_test.q` – Tests parser (main moved out of parser.q)
- `compiler/semantic_test.q` – Tests semantic
- `compiler/codegen_test.q` – Tests codegen
- `compiler/input.q` – Default input for bootstrap (contains `42`)

### How to Run

```powershell
# Build the bootstrap compiler (Rust compiles Q++)
cargo run -- build compiler/main.q

# Run bootstrap compiler from compiler/ (reads input.q, writes build/output.c)
cd compiler
.\build\output.exe
```

The bootstrap compiler produces valid C (e.g. `#include <stdio.h>\nint main(void) { long _r = 42; printf("%ld\n", _r); return 0; }`).

---

## What Needs to Be Done Next

### Done: Parser Expansion (binary, precedence, parentheses)

- **Binary expressions** – Parse `1 + 2`, `x * 3` (PLUS, MINUS, STAR, SLASH)
- **Precedence** – `*` and `/` bind tighter than `+` and `-`
- **Parentheses** – `(1 + 2) * 3`
- Added `ast_helpers.new_expr_binary(left, op, right)`
- Added `ql_usize_to_ptr` / `ql_ptr_to_usize` for parse result indexing
- Rust codegen: forward declarations for recursive module functions

### Medium-Term: Functions & Statements

- Parse `craft main() -> void { ... }`
- Parse `make x = 42;`, `check`, `otherwise`, `loopwhile`
- Extend AST (Stmt, FnDef, Program)
- Extend semantic (symbol table for functions, variables)
- Extend codegen (emit function bodies, statements)

### Long-Term: Full Self-Hosting

- Support the full Q++ syntax used by `compiler/main.q` and the rest of the compiler
- Bootstrap compiler compiles itself: `qpp build compiler/main.q` runs the Q++ bootstrap binary instead of the Rust compiler
- Eventually retire the Rust compiler for Q++ compilation

---

## Key Paths

| Path | Purpose |
|------|---------|
| `compiler/lexer.q` | Lexer |
| `compiler/parser.q` | Parser |
| `compiler/ast.q` | AST types |
| `compiler/semantic.q` | Semantic analysis |
| `compiler/codegen.q` | C codegen |
| `compiler/main.q` | Driver |
| `compiler/input.q` | Default input |
| `stdlib/vec.q`, `stdlib/map.q`, `stdlib/fmt.q` | Stdlib |
| `src/codegen/c.rs` | Rust C codegen (AST runtime, string escaping) |
| `src/main.rs` | Rust build driver (`base_dir` logic) |

---

## Build Commands

```powershell
# Build from project root (uses qpp.toml entry)
qpp build .

# Build a specific file (base_dir auto-resolves to project root)
cargo run -- build compiler/semantic_test.q
cargo run -- build compiler/codegen_test.q
cargo run -- build compiler/main.q
```

---

## Notes

- Parser has no `main`; use `compiler/parser_test.q` to test it
- `compiler/main.q` writes to `build/output.c` (relative to cwd; run from `compiler/`)
- Bootstrap compiler does not invoke the C compiler; it only emits C
- Map uses `str -> link void`; semantic uses two parallel vecs (names, types) for the symbol table
