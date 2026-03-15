extern craft puts(s: str) -> i32;
realm tokens {
    eternal CRAFT: i32 = 0;
    eternal MAKE: i32 = 1;
    craft foo() -> void {
        send;
    }
}
craft main() -> void {
    send 0;
}
