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
extern craft ql_usize_to_ptr(u: usize) -> link void;
extern craft ql_ptr_to_usize(p: link void) -> usize;

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

    craft new_expr_binary(left: link void, op: i32, right: link void) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_BINARY);
        ql_ast_expr_set_int(p, op as i64);
        ql_ast_expr_set_left(p, left);
        ql_ast_expr_set_right(p, right);
        send p;
    }
}

realm parser {
    // parse_expr: add + term | add - term. Returns vec [expr, next] or 0 (defined first for recursion)
    craft parse_expr(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_mul(toks, i);
        check (left_result == 0) {
            send 0;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx + (1 as usize) < n) {
            make tok_op: link void = vec.ptr_get(toks, idx);
            make ty_op: i32 = lexer.token_ty(tok_op);
            check (ty_op != tokens.PLUS && ty_op != tokens.MINUS) {
                stop;
            }
            make right_result: link void = parse_mul(toks, idx + (1 as usize));
            check (right_result == 0) {
                stop;
            }
            make right: link void = vec.ptr_get(right_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(right_result, 1));
            left = ast_helpers.new_expr_binary(left, ty_op, right);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, left);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
    }

    // parse_primary: returns vec [expr, next_index_as_ptr] or 0 on failure
    craft parse_primary(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i >= n) {
            send 0;
        }
        make tok: link void = vec.ptr_get(toks, i);
        make ty: i32 = lexer.token_ty(tok);
        check (ty == tokens.LPAREN) {
            make sub: link void = parse_expr(toks, i + (1 as usize));
            check (sub == 0) {
                send 0;
            }
            make expr: link void = vec.ptr_get(sub, 0);
            make next: usize = ql_ptr_to_usize(vec.ptr_get(sub, 1));
            check (next >= n) {
                send 0;
            }
            make rp: link void = vec.ptr_get(toks, next);
            check (lexer.token_ty(rp) != tokens.RPAREN) {
                send 0;
            }
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, expr);
            vec.ptr_push(result, ql_usize_to_ptr(next + (1 as usize)));
            send result;
        }
        check (ty == tokens.INT) {
            make val: i64 = lexer.token_int(tok);
            make e: link void = ast_helpers.new_expr_literal(val);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, e);
            vec.ptr_push(result, ql_usize_to_ptr(i + (1 as usize)));
            send result;
        }
        check (ty == tokens.IDENT) {
            make name: str = lexer.token_str(tok);
            make e: link void = ast_helpers.new_expr_ident(name);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, e);
            vec.ptr_push(result, ql_usize_to_ptr(i + (1 as usize)));
            send result;
        }
        send 0;
    }

    // parse_mul: term * factor | term / factor. Returns vec [expr, next] or 0
    craft parse_mul(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_primary(toks, i);
        check (left_result == 0) {
            send 0;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx + (1 as usize) < n) {
            make tok_op: link void = vec.ptr_get(toks, idx);
            make ty_op: i32 = lexer.token_ty(tok_op);
            check (ty_op != tokens.STAR && ty_op != tokens.SLASH) {
                stop;
            }
            make right_result: link void = parse_primary(toks, idx + (1 as usize));
            check (right_result == 0) {
                stop;
            }
            make right: link void = vec.ptr_get(right_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(right_result, 1));
            left = ast_helpers.new_expr_binary(left, ty_op, right);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, left);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
    }

    // Parse source string, return AST (link void = Expr or 0)
    craft parse(source: str) -> link void {
        make toks: link void = lexer.tokenize(source);
        make n: usize = vec.ptr_len(toks);
        check (n == 0) {
            send 0;
        }
        make parsed: link void = parse_expr(toks, 0);
        check (parsed == 0) {
            send 0;
        }
        make expr: link void = vec.ptr_get(parsed, 0);
        send expr;
    }
}
