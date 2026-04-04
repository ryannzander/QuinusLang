// Test semantic analysis - build: qpp build compiler/semantic_test.q

bring "vec";
bring "compiler.lexer";
bring "compiler.parser";
bring "compiler.semantic";

craft main() -> void {
    make src: str = "42";
    make ast: link void = parser.parse(src);
    check (ast == 0) {
        writeln("Parse failed");
        send;
    }
    make names: link void = vec.ptr_new();
    make types: link void = vec.ptr_new();
    make ty: str = semantic.check_expr(ast, names, types);
    check (lexer.str_eq(ty, "i64")) {
        writeln("Semantic OK: 42 -> i64");
    }
    otherwise {
        writeln("Semantic failed: expected i64");
    }
    vec.ptr_free(names);
    vec.ptr_free(types);
    send;
}
