// QuinusLang bootstrap compiler driver
// Pipeline: read source -> lex -> parse -> semantic -> codegen -> write C -> invoke cc
// Usage: build this, then run with input file (default: input.q)
// Build: cargo run -- build compiler/main.q

bring "fs";
bring "vec";
bring "compiler.lexer";
bring "compiler.parser";
bring "compiler.semantic";
bring "compiler.codegen";
bring "os";

extern craft strlen(s: str) -> usize;

craft main() -> void {
    make path: str = "input.q";
    make f: link void = fs.open_file(path, "r");
    check (f == 0) {
        writeln("Cannot open input.q");
        send;
    }
    make src: str = fs.read_all(f);
    fs.close(f);
    make ast: link void = parser.parse(src);
    check (ast == 0) {
        writeln("Parse failed");
        send;
    }
    make names: link void = vec.ptr_new();
    make types: link void = vec.ptr_new();
    make ty: str = semantic.check_expr(ast, names, types);
    check (strlen(ty) == 0) {
        writeln("Semantic check failed");
        send;
    }
    make c_code: str = codegen.emit_program(ast);
    os.run("mkdir build 2>nul");
    make out_path: str = "build/output.c";
    make ok: i32 = fs.write_all(out_path, c_code);
    check (ok != 1) {
        writeln("Cannot write output.c");
        send;
    }
    writeln("Compiled to build/output.c");
    send;
}
