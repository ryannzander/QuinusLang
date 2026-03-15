// Test codegen - build: cargo run -- build compiler/codegen_test.q

bring "compiler.parser";
bring "compiler.codegen";

craft main() -> void {
    make src: str = "42";
    make ast: link void = parser.parse(src);
    check (ast == 0) {
        writeln("Parse failed");
        send;
    }
    make c_code: str = codegen.emit_program(ast);
    writeln("Codegen OK");
    send;
}
