craft main() -> void {
    make x = 42;
    make y = 10;
    check (x > 0) {
        make z = x + y;
        send;
    }
    send;
}
