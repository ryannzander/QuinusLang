# std.map

Map/dictionary (string keys, pointer values).

## Functions

| Function | Signature | Description |
|----------|-----------|-------------|
| `str_ptr_new` | `() -> link void` | Create new map. |
| `str_ptr_put` | `(m: link void, key: str, value: link void) -> void` | Insert or update. |
| `str_ptr_get` | `(m: link void, key: str) -> link void` | Get value. |
| `str_ptr_has` | `(m: link void, key: str) -> bool` | Check if key exists. |
| `str_ptr_len` | `(m: link void) -> usize` | Number of entries. |
| `str_ptr_free` | `(m: link void) -> void` | Free map. |
