// Q++ standard library: Growable arrays
// VecPtr: array of void* (for AST nodes, tokens)
// VecU8: growable byte buffer (for string building)

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;
extern craft realloc(ptr: link void, size: usize) -> link void;
extern craft ql_vec_ptr_new() -> link void;
extern craft ql_vec_ptr_push(v: link void, ptr: link void) -> void;
extern craft ql_vec_ptr_get(v: link void, i: usize) -> link void;
extern craft ql_vec_ptr_len(v: link void) -> usize;
extern craft ql_vec_ptr_clear(v: link void) -> void;
extern craft ql_vec_ptr_free(v: link void) -> void;
extern craft ql_vec_u8_new() -> link void;
extern craft ql_vec_u8_push(v: link void, b: u8) -> void;
extern craft ql_vec_u8_append(v: link void, s: str) -> void;
extern craft ql_vec_u8_len(v: link void) -> usize;
extern craft ql_vec_u8_to_str(v: link void) -> str;
extern craft ql_vec_u8_clear(v: link void) -> void;
extern craft ql_vec_u8_free(v: link void) -> void;

realm vec {
    craft ptr_new() -> link void {
        send ql_vec_ptr_new();
    }

    craft ptr_push(v: link void, ptr: link void) -> void {
        ql_vec_ptr_push(v, ptr);
        send;
    }

    craft ptr_get(v: link void, i: usize) -> link void {
        send ql_vec_ptr_get(v, i);
    }

    craft ptr_len(v: link void) -> usize {
        send ql_vec_ptr_len(v);
    }

    craft ptr_clear(v: link void) -> void {
        ql_vec_ptr_clear(v);
        send;
    }

    craft ptr_free(v: link void) -> void {
        ql_vec_ptr_free(v);
        send;
    }

    craft u8_new() -> link void {
        send ql_vec_u8_new();
    }

    craft u8_push(v: link void, b: u8) -> void {
        ql_vec_u8_push(v, b);
        send;
    }

    craft u8_append(v: link void, s: str) -> void {
        ql_vec_u8_append(v, s);
        send;
    }

    craft u8_len(v: link void) -> usize {
        send ql_vec_u8_len(v);
    }

    craft u8_to_str(v: link void) -> str {
        send ql_vec_u8_to_str(v);
    }

    craft u8_clear(v: link void) -> void {
        ql_vec_u8_clear(v);
        send;
    }

    craft u8_free(v: link void) -> void {
        ql_vec_u8_free(v);
        send;
    }
}
