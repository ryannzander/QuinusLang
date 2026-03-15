extern craft ql_token_create(ty: i32, line: usize, col: usize, str_val: str, int_val: i64) -> link void;
extern craft strlen(s: str) -> usize;

realm test {
    craft foo() -> i64 {
        send 42;
    }
}

craft main() -> void {
    send;
}
