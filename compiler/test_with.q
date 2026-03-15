bring "std.fs";
bring "std.io";

craft main() -> void {
    with f = fs.open_file("compiler/test_with.q", "r") {
        make content: str = fs.read_all(f);
        write(content);
    }
    send;
}
