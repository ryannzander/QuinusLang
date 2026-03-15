extern craft puts(s: str) -> i32;
realm foo {
    craft bar() -> void { send; }
}
craft main() -> void {
    make x: i32 = 0;
    check (x == 0 && x < 1) {
        send;
    }
    send 0;
}
