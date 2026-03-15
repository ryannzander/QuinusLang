// QuinusLang standard library: Linear-search map (str -> link void)
// For symbol tables; O(n) lookup, no generics

extern craft ql_map_str_ptr_new() -> link void;
extern craft ql_map_str_ptr_put(m: link void, key: str, value: link void) -> void;
extern craft ql_map_str_ptr_get(m: link void, key: str) -> link void;
extern craft ql_map_str_ptr_has(m: link void, key: str) -> bool;
extern craft ql_map_str_ptr_len(m: link void) -> usize;
extern craft ql_map_str_ptr_free(m: link void) -> void;

realm map {
    craft str_ptr_new() -> link void {
        send ql_map_str_ptr_new();
    }

    craft str_ptr_put(m: link void, key: str, value: link void) -> void {
        ql_map_str_ptr_put(m, key, value);
        send;
    }

    craft str_ptr_get(m: link void, key: str) -> link void {
        send ql_map_str_ptr_get(m, key);
    }

    craft str_ptr_has(m: link void, key: str) -> bool {
        send ql_map_str_ptr_has(m, key);
    }

    craft str_ptr_len(m: link void) -> usize {
        send ql_map_str_ptr_len(m);
    }

    craft str_ptr_free(m: link void) -> void {
        ql_map_str_ptr_free(m);
        send;
    }
}
