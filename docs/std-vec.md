# std.vec

Dynamic vectors (growable arrays).

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `ptr_new` | `() -> link void` | Create new vector of pointers. |
| `ptr_push` | `(v: link void, ptr: link void) -> void` | Push element. |
| `ptr_get` | `(v: link void, i: usize) -> link void` | Get element at index. |
| `ptr_len` | `(v: link void) -> usize` | Length. |
| `ptr_clear` | `(v: link void) -> void` | Clear vector. |
| `ptr_free` | `(v: link void) -> void` | Free vector. |

Similar `u8_*` variants exist for byte vectors.
