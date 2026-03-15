extern craft puts(s: str) -> i32;
realm foo {
    craft bar() -> void {
        send;
    }
}
extern craft strlen(s: str) -> usize;
craft main() -> void {
    send 0;
}
