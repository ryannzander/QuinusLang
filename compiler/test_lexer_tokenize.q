extern craft ql_token_create(ty: i32, line: usize, col: usize, str_val: str, int_val: i64) -> link void;
extern craft ql_token_ty(t: link void) -> i32;
extern craft ql_str_at(s: str, i: usize) -> i32;
extern craft strlen(s: str) -> usize;
realm fs { craft open_file(path: str, mode: str) -> link void { send 0; } }
realm vec {
    craft ptr_new() -> link void { send 0; }
    craft ptr_push(v: link void, ptr: link void) -> void { send; }
}
realm tokens {
    eternal EOF: i32 = 34;
}
realm lexer {
    craft token_ty(t: link void) -> i32 { send ql_token_ty(t); }
    craft tokenize(source: str) -> link void {
        make tok_list: link void = vec.ptr_new();
        check (source == 0) {
            vec.ptr_push(tok_list, ql_token_create(tokens.EOF, 1, 1, "", 0));
            send tok_list;
        }
        send tok_list;
    }
}
extern craft puts(s: str) -> i32;
craft main() -> void { send 0; }
