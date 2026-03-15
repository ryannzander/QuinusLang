bring "fs";
bring "os";
bring "compiler.preprocess";

craft main() -> void {
    make base: str = ".";
    make src: str = "realm test { craft x() -> i64 { send 42; } }
";
    make flat: str = preprocess.flatten(src, base);
    writeln(flat);
    send;
}
