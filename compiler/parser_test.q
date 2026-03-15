// Test parser - build: cargo run -- build compiler/parser_test.q

bring "vec";
bring "compiler.lexer";
bring "compiler.parser";

craft main() -> void {
    make src: str = "1 + 2 * 3";
    make ast: link void = parser.parse(src);
    check (ast == 0) {
        writeln("Parse failed");
    }
    otherwise {
        writeln("Parser OK (1+2*3)");
    }
    send;
}
