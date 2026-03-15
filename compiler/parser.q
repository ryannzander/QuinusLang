// QuinusLang parser - minimal recursive descent
// Reads tokens from lexer, produces AST (subset: int, ident, binary +)
// Build: quinus build compiler/parser.q

bring "fs";
bring "vec";
bring "compiler.lexer";
bring "compiler.tokens";
bring "compiler.ast";

// AST node constructors - must be before parser which calls them
extern craft ql_ast_expr_alloc() -> link void;
extern craft ql_ast_expr_set_tag(p: link void, tag: i32) -> void;
extern craft ql_ast_expr_set_int(p: link void, val: i64) -> void;
extern craft ql_ast_expr_set_str(p: link void, s: str) -> void;
extern craft ql_ast_expr_set_left(p: link void, left: link void) -> void;
extern craft ql_ast_expr_set_right(p: link void, right: link void) -> void;

realm ast_helpers {
    craft new_expr_literal(val: i64) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_LITERAL);
        ql_ast_expr_set_int(p, val);
        send p;
    }

    craft new_expr_ident(name: str) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_IDENT);
        ql_ast_expr_set_str(p, name);
        send p;
    }
}

realm parser {
    // parse_expr: tokens, start index. Returns expr (link void)
    // Minimal: parse INT or IDENT. Must be defined before parse (which calls it).
    craft parse_expr(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i >= n) {
            send 0;
        }
        make tok: link void = vec.ptr_get(toks, i);
        make ty: i32 = lexer.token_ty(tok);
        check (ty == tokens.INT) {
            make val: i64 = lexer.token_int(tok);
            make e: link void = ast_helpers.new_expr_literal(val);
            send e;
        }
        check (ty == tokens.IDENT) {
            make name: str = lexer.token_str(tok);
            make e: link void = ast_helpers.new_expr_ident(name);
            send e;
        }
        send 0;
    }

    // Parse source string, return AST (link void = Expr or 0)
    craft parse(source: str) -> link void {
        make toks: link void = lexer.tokenize(source);
        make n: usize = vec.ptr_len(toks);
        check (n == 0) {
            send 0;
        }
        make expr: link void = parse_expr(toks, 0);
        send expr;
    }
}

craft main() -> void {
    // Test with simple input: "42" or "x" parses as literal/ident
    make src: str = "42";
    make ast: link void = parser.parse(src);
    check (ast == 0) {
        writeln("Parse failed");
    }
    otherwise {
        writeln("Parser OK");
    }
    send;
}
