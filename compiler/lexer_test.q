// Test lexer - has main for standalone build
bring "fs";
bring "vec";
bring "compiler.lexer";

craft main() -> void {
    make path: str = "compiler/lexer.q";
    make f: link void = fs.open_file(path, "r");
    check (f == 0) {
        writeln("Cannot open file");
        send;
    }
    make src: str = fs.read_all(f);
    fs.close(f);
    make toks: link void = lexer.tokenize(src);
    make n: usize = vec.ptr_len(toks);
    writeln("Lexer OK");
    vec.ptr_free(toks);
    send;
}
