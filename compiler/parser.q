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

    // parse_var_decl: make ident = expr; Returns vec [name, expr] or 0
    craft parse_var_decl(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i + (4 as usize) >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        make t1: link void = vec.ptr_get(toks, i + (1 as usize));
        make t2: link void = vec.ptr_get(toks, i + (2 as usize));
        check (lexer.token_ty(t0) != tokens.MAKE) {
            send 0;
        }
        check (lexer.token_ty(t1) != tokens.IDENT) {
            send 0;
        }
        check (lexer.token_ty(t2) != tokens.EQ) {
            send 0;
        }
        make name: str = lexer.token_str(t1);
        make expr_result: link void = parse_expr(toks, i + (3 as usize));
        check (expr_result == 0) {
            send 0;
        }
        make expr: link void = vec.ptr_get(expr_result, 0);
        make next: usize = ql_ptr_to_usize(vec.ptr_get(expr_result, 1));
        check (next >= n) {
            send 0;
        }
        make tok_semi: link void = vec.ptr_get(toks, next);
        check (lexer.token_ty(tok_semi) != tokens.SEMICOLON) {
            send 0;
        }
        make pair: link void = vec.ptr_new();
        vec.ptr_push(pair, name);
        vec.ptr_push(pair, expr);
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, pair);
        vec.ptr_push(result, ql_usize_to_ptr(next + (1 as usize)));
        send result;
    }

    // skip_brings: advance idx past bring "x";*
    craft skip_brings(toks: link void, start: usize) -> usize {
        make n: usize = vec.ptr_len(toks);
        make shift idx: usize = start;
        loopwhile (idx + (2 as usize) < n) {
            make t0: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t0) != tokens.BRING) {
                stop;
            }
            make t1: link void = vec.ptr_get(toks, idx + (1 as usize));
            check (lexer.token_ty(t1) != tokens.STR) {
                stop;
            }
            make t2: link void = vec.ptr_get(toks, idx + (2 as usize));
            check (lexer.token_ty(t2) != tokens.SEMICOLON) {
                stop;
            }
            idx = idx + (3 as usize);
        }
        send idx;
    }

    // skip_externs: advance idx past extern craft ...;*
    craft skip_externs(toks: link void, start: usize) -> usize {
        make n: usize = vec.ptr_len(toks);
        make shift idx: usize = start;
        loopwhile (idx < n) {
            make t: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t) != tokens.EXTERN) {
                stop;
            }
            idx = idx + (1 as usize);
            loopwhile (idx < n) {
                make t2: link void = vec.ptr_get(toks, idx);
                idx = idx + (1 as usize);
                check (lexer.token_ty(t2) == tokens.SEMICOLON) {
                    stop;
                }
            }
        }
        send idx;
    }

    // parse_block: { stmt* } Returns vec [next_idx_ptr, stmts_vec]
    // stmt = vec [tag, ...] tag: STMT_VAR=10, STMT_IF=12, STMT_WHILE=13, STMT_RETURN=14, STMT_EXPR=15
    craft parse_block(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i >= n) {
            send 0;
        }
        make lb: link void = vec.ptr_get(toks, i);
        check (lexer.token_ty(lb) != tokens.LBRACE) {
            send 0;
        }
        make stmts: link void = vec.ptr_new();
        make shift idx: usize = i + (1 as usize);
        loopwhile (idx < n) {
            make t: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t) == tokens.RBRACE) {
                idx = idx + (1 as usize);
                stop;
            }
            make stmt_result: link void = parse_stmt(toks, idx);
            check (stmt_result == 0) {
                send 0;
            }
            make stmt: link void = vec.ptr_get(stmt_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(stmt_result, 1));
            vec.ptr_push(stmts, stmt);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        vec.ptr_push(result, stmts);
        send result;
    }

    // parse_stmt: returns vec [stmt, next_idx_ptr]. stmt = vec [tag, ...]
    craft parse_stmt(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        make ty: i32 = lexer.token_ty(t0);
        check (ty == tokens.CHECK) {
            check (i + (5 as usize) >= n) {
                send 0;
            }
            make lp: link void = vec.ptr_get(toks, i + (1 as usize));
            check (lexer.token_ty(lp) != tokens.LPAREN) {
                send 0;
            }
            make cond_result: link void = parse_expr(toks, i + (2 as usize));
            check (cond_result == 0) {
                send 0;
            }
            make cond: link void = vec.ptr_get(cond_result, 0);
            make next: usize = ql_ptr_to_usize(vec.ptr_get(cond_result, 1));
            check (next >= n) {
                send 0;
            }
            make rp: link void = vec.ptr_get(toks, next);
            check (lexer.token_ty(rp) != tokens.RPAREN) {
                send 0;
            }
            make block_result: link void = parse_block(toks, next + (1 as usize));
            check (block_result == 0) {
                send 0;
            }
            make body: link void = vec.ptr_get(block_result, 1);
            make stmt: link void = vec.ptr_new();
            vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_IF as i64) as usize));
            vec.ptr_push(stmt, cond);
            vec.ptr_push(stmt, body);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, stmt);
            vec.ptr_push(result, ql_usize_to_ptr(ql_ptr_to_usize(vec.ptr_get(block_result, 0))));
            send result;
        }
        check (ty == tokens.LOOPWHILE) {
            check (i + (5 as usize) >= n) {
                send 0;
            }
            make lp: link void = vec.ptr_get(toks, i + (1 as usize));
            check (lexer.token_ty(lp) != tokens.LPAREN) {
                send 0;
            }
            make cond_result: link void = parse_expr(toks, i + (2 as usize));
            check (cond_result == 0) {
                send 0;
            }
            make cond: link void = vec.ptr_get(cond_result, 0);
            make next: usize = ql_ptr_to_usize(vec.ptr_get(cond_result, 1));
            check (next >= n) {
                send 0;
            }
            make rp: link void = vec.ptr_get(toks, next);
            check (lexer.token_ty(rp) != tokens.RPAREN) {
                send 0;
            }
            make block_result: link void = parse_block(toks, next + (1 as usize));
            check (block_result == 0) {
                send 0;
            }
            make body: link void = vec.ptr_get(block_result, 1);
            make stmt: link void = vec.ptr_new();
            vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_WHILE as i64) as usize));
            vec.ptr_push(stmt, cond);
            vec.ptr_push(stmt, body);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, stmt);
            vec.ptr_push(result, ql_usize_to_ptr(ql_ptr_to_usize(vec.ptr_get(block_result, 0))));
            send result;
        }
        check (ty == tokens.SEND) {
            make expr_result: link void = parse_expr(toks, i + (1 as usize));
            make shift ret_expr: link void = 0 as link void;
            make shift next: usize = i + (1 as usize);
            check (expr_result != 0) {
                ret_expr = vec.ptr_get(expr_result, 0);
                next = ql_ptr_to_usize(vec.ptr_get(expr_result, 1));
            }
            check (next >= n) {
                send 0;
            }
            make semi: link void = vec.ptr_get(toks, next);
            check (lexer.token_ty(semi) != tokens.SEMICOLON) {
                send 0;
            }
            make stmt: link void = vec.ptr_new();
            vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_RETURN as i64) as usize));
            vec.ptr_push(stmt, ret_expr);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, stmt);
            vec.ptr_push(result, ql_usize_to_ptr(next + (1 as usize)));
            send result;
        }
        make vd: link void = parse_var_decl(toks, i);
        check (vd != 0) {
            make pair: link void = vec.ptr_get(vd, 0);
            make next: usize = ql_ptr_to_usize(vec.ptr_get(vd, 1));
            make stmt: link void = vec.ptr_new();
            vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_VAR as i64) as usize));
            vec.ptr_push(stmt, pair);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, stmt);
            vec.ptr_push(result, ql_usize_to_ptr(next));
            send result;
        }
        make expr_result: link void = parse_expr(toks, i);
        check (expr_result == 0) {
            send 0;
        }
        make expr: link void = vec.ptr_get(expr_result, 0);
        make next: usize = ql_ptr_to_usize(vec.ptr_get(expr_result, 1));
        check (next >= n) {
            send 0;
        }
        make semi: link void = vec.ptr_get(toks, next);
        check (lexer.token_ty(semi) != tokens.SEMICOLON) {
            send 0;
        }
        make stmt: link void = vec.ptr_new();
        vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_EXPR as i64) as usize));
        vec.ptr_push(stmt, expr);
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, stmt);
        vec.ptr_push(result, ql_usize_to_ptr(next + (1 as usize)));
        send result;
    }

    // parse_fn_def: craft ident() -> ret_ty { block }
    craft parse_fn_def(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i + (7 as usize) >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        make t1: link void = vec.ptr_get(toks, i + (1 as usize));
        make t2: link void = vec.ptr_get(toks, i + (2 as usize));
        make t3: link void = vec.ptr_get(toks, i + (3 as usize));
        check (lexer.token_ty(t0) != tokens.CRAFT) {
            send 0;
        }
        check (lexer.token_ty(t1) != tokens.IDENT) {
            send 0;
        }
        check (lexer.token_ty(t2) != tokens.LPAREN) {
            send 0;
        }
        check (lexer.token_ty(t3) != tokens.RPAREN) {
            send 0;
        }
        make name: str = lexer.token_str(t1);
        make t4: link void = vec.ptr_get(toks, i + (4 as usize));
        check (lexer.token_ty(t4) != tokens.ARROW) {
            send 0;
        }
        make t5: link void = vec.ptr_get(toks, i + (5 as usize));
        check (lexer.token_ty(t5) != tokens.IDENT) {
            send 0;
        }
        make ret_ty: str = lexer.token_str(t5);
        make block_result: link void = parse_block(toks, i + (6 as usize));
        check (block_result == 0) {
            send 0;
        }
        make body: link void = vec.ptr_get(block_result, 1);
        make fn_def: link void = vec.ptr_new();
        vec.ptr_push(fn_def, name);
        vec.ptr_push(fn_def, ret_ty);
        vec.ptr_push(fn_def, body);
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, fn_def);
        vec.ptr_push(result, ql_usize_to_ptr(ql_ptr_to_usize(vec.ptr_get(block_result, 0))));
        send result;
    }

    // Parse source: [bring]* [extern]* (craft fn | script)
    // Returns: for craft: vec ["fn", fn_def]. For script: vec ["script", stmts, result_expr]
    craft parse(source: str) -> link void {
        make toks: link void = lexer.tokenize(source);
        make n: usize = vec.ptr_len(toks);
        check (n == 0) {
            send 0;
        }
        make shift idx: usize = skip_brings(toks, 0);
        idx = skip_externs(toks, idx);
        check (idx >= n) {
            send 0;
        }
        make t: link void = vec.ptr_get(toks, idx);
        check (lexer.token_ty(t) == tokens.CRAFT) {
            make fn_result: link void = parse_fn_def(toks, idx);
            check (fn_result == 0) {
                send 0;
            }
            make fn_def: link void = vec.ptr_get(fn_result, 0);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, "fn");
            vec.ptr_push(result, fn_def);
            send result;
        }
        make stmts: link void = vec.ptr_new();
        make shift i: usize = idx;
        loopwhile (i < n) {
            make vd: link void = parse_var_decl(toks, i);
            check (vd == 0) {
                stop;
            }
            make pair: link void = vec.ptr_get(vd, 0);
            i = ql_ptr_to_usize(vec.ptr_get(vd, 1));
            vec.ptr_push(stmts, pair);
        }
        make expr_result: link void = parse_expr(toks, i);
        check (expr_result == 0) {
            send 0;
        }
        make result_expr: link void = vec.ptr_get(expr_result, 0);
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, "script");
        vec.ptr_push(result, stmts);
        vec.ptr_push(result, result_expr);
        send result;
    }
}
