extern craft puts(s: str) -> i32;
realm ast {
    form Expr {
        tag: i32,
        left: link void,
        right: link void
    }
    craft foo() -> void {
        send;
    }
}
craft main() -> void {
    send 0;
}
