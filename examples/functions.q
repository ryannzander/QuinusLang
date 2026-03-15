craft add(a: int, b: int) -> int {
    send (a + b);
}

craft main() -> void {
    make shift x: int = add(1, 2);
    print(x);
    send;
}
