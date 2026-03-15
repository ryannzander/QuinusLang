extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;
realm fs {
    craft open_file(path: str, mode: str) -> link void { send fopen(path, mode); }
    craft close(stream: link void) -> i32 { send fclose(stream); }
}
extern craft malloc(size: usize) -> link void;
extern craft ql_vec_ptr_new() -> link void;
extern craft ql_vec_ptr_push(v: link void, ptr: link void) -> void;
extern craft ql_vec_ptr_get(v: link void, i: usize) -> link void;
extern craft ql_vec_ptr_len(v: link void) -> usize;
realm vec {
    craft ptr_new() -> link void { send ql_vec_ptr_new(); }
    craft ptr_push(v: link void, ptr: link void) -> void { ql_vec_ptr_push(v, ptr); send; }
    craft ptr_get(v: link void, i: usize) -> link void { send ql_vec_ptr_get(v, i); }
    craft ptr_len(v: link void) -> usize { send ql_vec_ptr_len(v); }
}
realm tokens {
    eternal CRAFT: i32 = 0;
    eternal MAKE: i32 = 1;
}
extern craft strlen(s: str) -> usize;
craft main() -> void { send 0; }
