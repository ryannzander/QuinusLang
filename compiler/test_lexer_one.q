extern craft ql_token_ty(t: link void) -> i32;
realm fs {
    craft open_file(path: str, mode: str) -> link void { send 0; }
}
realm vec {
    craft ptr_new() -> link void { send 0; }
}
realm tokens {
    eternal EOF: i32 = 34;
}
realm lexer {
    craft token_ty(t: link void) -> i32 {
        send ql_token_ty(t);
    }
}
extern craft strlen(s: str) -> usize;
craft main() -> void { send 0; }
