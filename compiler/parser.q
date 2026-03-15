// QuinusLang parser - minimal recursive descent
// Reads tokens from lexer, produces AST (subset: int, ident, binary +)
// Build: quinus build compiler/parser.q

bring "fs";
bring "vec";
bring "str";
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
extern craft ql_ast_expr_set_args(p: link void, args: link void) -> void;
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

    craft new_expr_call(callee: link void, args: link void) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_CALL);
        ql_ast_expr_set_left(p, callee);
        ql_ast_expr_set_args(p, args);
        send p;
    }

    craft new_expr_str(s: str) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_STR);
        ql_ast_expr_set_str(p, s);
        send p;
    }

    craft new_expr_cast(expr: link void, target_ty: str) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_CAST);
        ql_ast_expr_set_left(p, expr);
        ql_ast_expr_set_str(p, target_ty);
        send p;
    }

    craft new_expr_field(base: link void, field: str) -> link void {
        make p: link void = ql_ast_expr_alloc();
        ql_ast_expr_set_tag(p, ast.EXPR_FIELD);
        ql_ast_expr_set_left(p, base);
        ql_ast_expr_set_str(p, field);
        send p;
    }
}

realm parser {
    // parse_expr: top level, delegates to parse_compare
    craft parse_expr(toks: link void, i: usize) -> link void {
        send parse_compare(toks, i);
    }

    // parse_compare: add == add | add != add | add < add | etc.
    craft parse_compare(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_add(toks, i);
        check (left_result == 0) {
            send 0;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx + (1 as usize) < n) {
            make tok_op: link void = vec.ptr_get(toks, idx);
            make ty_op: i32 = lexer.token_ty(tok_op);
            check (ty_op != tokens.EQEQ && ty_op != tokens.NE && ty_op != tokens.LT && ty_op != tokens.LE && ty_op != tokens.GT && ty_op != tokens.GE) {
                stop;
            }
            make right_result: link void = parse_add(toks, idx + (1 as usize));
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

    // parse_add: [unary -] mul + mul | mul - mul
    craft parse_add(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        make left_result: link void = parse_mul(toks, i);
        check (left_result == 0) {
            check (i >= n || lexer.token_ty(vec.ptr_get(toks, i)) != tokens.MINUS) {
                send 0;
            }
            make right_result: link void = parse_mul(toks, i + (1 as usize));
            check (right_result == 0) {
                send 0;
            }
            make zero: link void = ast_helpers.new_expr_literal(0);
            make right: link void = vec.ptr_get(right_result, 0);
            make left: link void = ast_helpers.new_expr_binary(zero, tokens.MINUS, right);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, left);
            vec.ptr_push(result, vec.ptr_get(right_result, 1));
            send result;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
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

    // parse_postfix: primary then optional .ident or (args)
    craft parse_postfix(toks: link void, i: usize) -> link void {
        make base_result: link void = parse_primary(toks, i);
        check (base_result == 0) {
            send 0;
        }
        make shift base: link void = vec.ptr_get(base_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(base_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx < n) {
            make t: link void = vec.ptr_get(toks, idx);
            make ty: i32 = lexer.token_ty(t);
            check (ty == tokens.DOT) {
                check (idx + (2 as usize) >= n) {
                    stop;
                }
                make t1: link void = vec.ptr_get(toks, idx + (1 as usize));
                check (lexer.token_ty(t1) != tokens.IDENT) {
                    stop;
                }
                make field: str = lexer.token_str(t1);
                base = ast_helpers.new_expr_field(base, field);
                idx = idx + (2 as usize);
                skip;
            }
            check (ty == tokens.LPAREN) {
                make args_result: link void = parse_call_args(toks, idx);
                check (args_result == 0) {
                    stop;
                }
                make args: link void = vec.ptr_get(args_result, 0);
                idx = ql_ptr_to_usize(vec.ptr_get(args_result, 1));
                base = ast_helpers.new_expr_call(base, args);
                skip;
            }
            check (ty == tokens.IDENT) {
                make s: str = lexer.token_str(t);
                check (lexer.str_eq(s, "as") && idx + (2 as usize) <= n) {
                    make t1: link void = vec.ptr_get(toks, idx + (1 as usize));
                    check (lexer.token_ty(t1) == tokens.IDENT) {
                        make target: str = lexer.token_str(t1);
                        base = ast_helpers.new_expr_cast(base, target);
                        idx = idx + (2 as usize);
                        skip;
                    }
                }
            }
            stop;
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, base);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
    }

    // parse_call_args: ( expr, expr, ... ) Returns vec [args_vec, next_idx_ptr]
    craft parse_call_args(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i >= n) {
            send 0;
        }
        make lp: link void = vec.ptr_get(toks, i);
        check (lexer.token_ty(lp) != tokens.LPAREN) {
            send 0;
        }
        make args: link void = vec.ptr_new();
        make shift idx: usize = i + (1 as usize);
        check (idx >= n) {
            send 0;
        }
        make t: link void = vec.ptr_get(toks, idx);
        check (lexer.token_ty(t) == tokens.RPAREN) {
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, args);
            vec.ptr_push(result, ql_usize_to_ptr(idx + (1 as usize)));
            send result;
        }
        loopwhile (idx < n) {
            make expr_result: link void = parse_expr(toks, idx);
            check (expr_result == 0) {
                send 0;
            }
            make expr: link void = vec.ptr_get(expr_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(expr_result, 1));
            vec.ptr_push(args, expr);
            check (idx >= n) {
                send 0;
            }
            make t2: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t2) == tokens.RPAREN) {
                idx = idx + (1 as usize);
                stop;
            }
            check (lexer.token_ty(t2) != tokens.COMMA) {
                send 0;
            }
            idx = idx + (1 as usize);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, args);
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
        check (ty == tokens.STR) {
            make s: str = lexer.token_str(tok);
            make e: link void = ast_helpers.new_expr_str(s);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, e);
            vec.ptr_push(result, ql_usize_to_ptr(i + (1 as usize)));
            send result;
        }
        check (ty == tokens.BOOL) {
            make s: str = lexer.token_str(tok);
            make shift val: i64 = 0;
            check (lexer.str_eq(s, "true")) {
                val = 1;
            }
            make e: link void = ast_helpers.new_expr_literal(val);
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, e);
            vec.ptr_push(result, ql_usize_to_ptr(i + (1 as usize)));
            send result;
        }
        send 0;
    }

    // parse_mul: term * factor | term / factor. Returns vec [expr, next] or 0
    craft parse_mul(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_postfix(toks, i);
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
            make right_result: link void = parse_postfix(toks, idx + (1 as usize));
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

    // parse_var_decl: make [shift] ident [ : type ] = expr; Returns vec [name, expr] or 0
    craft parse_var_decl(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i + (4 as usize) >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        check (lexer.token_ty(t0) != tokens.MAKE) {
            send 0;
        }
        make shift t1: link void = vec.ptr_get(toks, i + (1 as usize));
        make shift j: usize = i + (2 as usize);
        check (lexer.token_ty(t1) == tokens.IDENT && lexer.str_eq(lexer.token_str(t1), "shift")) {
            check (j >= n) {
                send 0;
            }
            t1 = vec.ptr_get(toks, j);
            j = j + (1 as usize);
        }
        check (lexer.token_ty(t1) != tokens.IDENT) {
            send 0;
        }
        make name: str = lexer.token_str(t1);
        make shift t2: link void = vec.ptr_get(toks, j);
        check (lexer.token_ty(t2) == tokens.COLON) {
            j = j + (1 as usize);
            loopwhile (j < n) {
                make tj: link void = vec.ptr_get(toks, j);
                make ty: i32 = lexer.token_ty(tj);
                check (ty == tokens.EQ) {
                    stop;
                }
                check (ty == tokens.LINK || ty == tokens.IDENT) {
                    j = j + (1 as usize);
                    skip;
                }
                stop;
            }
            check (j >= n) {
                send 0;
            }
            t2 = vec.ptr_get(toks, j);
        }
        check (lexer.token_ty(t2) != tokens.EQ) {
            send 0;
        }
        j = j + (1 as usize);
        make expr_result: link void = parse_expr(toks, j);
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

    // parse_externs: extern craft name ( params ) -> ret ;*
    // extern = vec [name, ret_ty]. Params skipped for now.
    craft parse_externs(toks: link void, start: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        make externs: link void = vec.ptr_new();
        make shift idx: usize = start;
        loopwhile (idx + (6 as usize) < n) {
            make t0: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t0) != tokens.EXTERN) {
                stop;
            }
            make t1: link void = vec.ptr_get(toks, idx + (1 as usize));
            make t2: link void = vec.ptr_get(toks, idx + (2 as usize));
            check (lexer.token_ty(t1) != tokens.CRAFT) {
                stop;
            }
            check (lexer.token_ty(t2) != tokens.IDENT) {
                stop;
            }
            make name: str = lexer.token_str(t2);
            make shift j: usize = idx + (3 as usize);
            check (j >= n) {
                stop;
            }
            make t3: link void = vec.ptr_get(toks, j);
            check (lexer.token_ty(t3) != tokens.LPAREN) {
                stop;
            }
            j = j + (1 as usize);
            make shift depth: i32 = 1;
            loopwhile (j < n && depth > 0) {
                make tj: link void = vec.ptr_get(toks, j);
                make ty: i32 = lexer.token_ty(tj);
                check (ty == tokens.LPAREN) {
                    depth = depth + 1;
                }
                check (ty == tokens.RPAREN) {
                    depth = depth - 1;
                }
                j = j + (1 as usize);
            }
            check (j >= n) {
                stop;
            }
            make t_arrow: link void = vec.ptr_get(toks, j);
            check (lexer.token_ty(t_arrow) != tokens.ARROW) {
                stop;
            }
            j = j + (1 as usize);
            make ret_parts: link void = vec.ptr_new();
            loopwhile (j < n) {
                make t_cur: link void = vec.ptr_get(toks, j);
                make ty_cur: i32 = lexer.token_ty(t_cur);
                check (ty_cur == tokens.SEMICOLON) {
                    j = j + (1 as usize);
                    stop;
                }
                check (ty_cur == tokens.LINK || ty_cur == tokens.IDENT) {
                    vec.ptr_push(ret_parts, lexer.token_str(t_cur));
                    j = j + (1 as usize);
                    skip;
                }
                stop;
            }
            make np: usize = vec.ptr_len(ret_parts);
            check (np == 0) {
                stop;
            }
            make shift ret_ty: str = vec.ptr_get(ret_parts, 0) as str;
            check (np == 2) {
                make a: str = vec.ptr_get(ret_parts, 0) as str;
                make b: str = vec.ptr_get(ret_parts, 1) as str;
                ret_ty = str.concat(a, str.concat(" ", b));
            }
            make ext: link void = vec.ptr_new();
            vec.ptr_push(ext, name);
            vec.ptr_push(ext, ret_ty);
            vec.ptr_push(externs, ext);
            idx = j;
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, externs);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
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

    // parse_realm_def: realm name { craft* }
    // Returns vec [realm_name, crafts_vec] or 0. crafts_vec = vec of fn_def.
    craft parse_realm_def(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i + (4 as usize) >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        make t1: link void = vec.ptr_get(toks, i + (1 as usize));
        make t2: link void = vec.ptr_get(toks, i + (2 as usize));
        check (lexer.token_ty(t0) != tokens.REALM) {
            send 0;
        }
        check (lexer.token_ty(t1) != tokens.IDENT) {
            send 0;
        }
        check (lexer.token_ty(t2) != tokens.LBRACE) {
            send 0;
        }
        make name: str = lexer.token_str(t1);
        make crafts: link void = vec.ptr_new();
        make shift idx: usize = i + (3 as usize);
        loopwhile (idx < n) {
            make t: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(t) == tokens.RBRACE) {
                idx = idx + (1 as usize);
                stop;
            }
            check (lexer.token_ty(t) == tokens.IDENT && lexer.str_eq(lexer.token_str(t), "eternal")) {
                loopwhile (idx < n) {
                    make tt: link void = vec.ptr_get(toks, idx);
                    idx = idx + (1 as usize);
                    check (lexer.token_ty(tt) == tokens.SEMICOLON) {
                        stop;
                    }
                }
                skip;
            }
            check (lexer.token_ty(t) == tokens.FORM) {
                check (idx + (3 as usize) >= n) {
                    send 0;
                }
                make t_lb: link void = vec.ptr_get(toks, idx + (2 as usize));
                check (lexer.token_ty(t_lb) != tokens.LBRACE) {
                    send 0;
                }
                make shift depth: i32 = 1;
                idx = idx + (3 as usize);
                loopwhile (idx < n && depth > 0) {
                    make tt: link void = vec.ptr_get(toks, idx);
                    idx = idx + (1 as usize);
                    check (lexer.token_ty(tt) == tokens.LBRACE) {
                        depth = depth + 1;
                    }
                    check (lexer.token_ty(tt) == tokens.RBRACE) {
                        depth = depth - 1;
                    }
                }
                skip;
            }
            make fn_result: link void = parse_fn_def(toks, idx);
            check (fn_result == 0) {
                send 0;
            }
            make fn_def: link void = vec.ptr_get(fn_result, 0);
            vec.ptr_push(crafts, fn_def);
            idx = ql_ptr_to_usize(vec.ptr_get(fn_result, 1));
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, name);
        vec.ptr_push(result, crafts);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
    }

    // parse_fn_def: craft ident ( params? ) -> ret_ty { block }
    // Skips params for now (finds closing paren)
    craft parse_fn_def(toks: link void, i: usize) -> link void {
        make n: usize = vec.ptr_len(toks);
        check (i + (6 as usize) >= n) {
            send 0;
        }
        make t0: link void = vec.ptr_get(toks, i);
        make t1: link void = vec.ptr_get(toks, i + (1 as usize));
        make t2: link void = vec.ptr_get(toks, i + (2 as usize));
        check (lexer.token_ty(t0) != tokens.CRAFT) {
            send 0;
        }
        check (lexer.token_ty(t1) != tokens.IDENT) {
            send 0;
        }
        check (lexer.token_ty(t2) != tokens.LPAREN) {
            send 0;
        }
        make name: str = lexer.token_str(t1);
        make shift j: usize = i + (3 as usize);
        make shift depth: i32 = 1;
        loopwhile (j < n && depth > 0) {
            make tj: link void = vec.ptr_get(toks, j);
            make ty: i32 = lexer.token_ty(tj);
            check (ty == tokens.LPAREN) {
                depth = depth + 1;
            }
            check (ty == tokens.RPAREN) {
                depth = depth - 1;
            }
            j = j + (1 as usize);
        }
        check (j + (3 as usize) >= n) {
            send 0;
        }
        make t_arrow: link void = vec.ptr_get(toks, j);
        check (lexer.token_ty(t_arrow) != tokens.ARROW) {
            send 0;
        }
        make ret_parts: link void = vec.ptr_new();
        j = j + (1 as usize);
        loopwhile (j < n) {
            make t_cur: link void = vec.ptr_get(toks, j);
            make ty_cur: i32 = lexer.token_ty(t_cur);
            check (ty_cur == tokens.LBRACE) {
                stop;
            }
            check (ty_cur == tokens.LINK || ty_cur == tokens.IDENT) {
                vec.ptr_push(ret_parts, lexer.token_str(t_cur));
                j = j + (1 as usize);
                skip;
            }
            stop;
        }
        make np: usize = vec.ptr_len(ret_parts);
        check (np == 0) {
            send 0;
        }
        make shift ret_ty: str = vec.ptr_get(ret_parts, 0) as str;
        check (np == 2) {
            make a: str = vec.ptr_get(ret_parts, 0) as str;
            make b: str = vec.ptr_get(ret_parts, 1) as str;
            ret_ty = str.concat(a, str.concat(" ", b));
        }
        make block_result: link void = parse_block(toks, j);
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

    // Parse source: [bring]* [extern]* (realm* craft* | craft | script)
    // Returns: vec [externs, kind, ...] where kind is "fn", "script", or "program"
    // program: vec [externs, "program", items] items = vec of ["realm", name, crafts] or ["craft", fn_def]
    craft parse(source: str) -> link void {
        make toks: link void = lexer.tokenize(source);
        make n: usize = vec.ptr_len(toks);
        check (n == 0) {
            send 0;
        }
        make shift idx: usize = skip_brings(toks, 0);
        make ext_result: link void = parse_externs(toks, idx);
        make externs: link void = vec.ptr_get(ext_result, 0);
        idx = ql_ptr_to_usize(vec.ptr_get(ext_result, 1));
        check (idx >= n) {
            send 0;
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, externs);
        make items: link void = vec.ptr_new();
        make shift i: usize = idx;
        loopwhile (i < n) {
            make t: link void = vec.ptr_get(toks, i);
            make ty: i32 = lexer.token_ty(t);
            check (ty == tokens.EXTERN) {
                make ext_result: link void = parse_externs(toks, i);
                make new_ext: link void = vec.ptr_get(ext_result, 0);
                make ext_count: usize = vec.ptr_len(new_ext);
                make shift k: usize = 0;
                loopwhile (k < ext_count) {
                    vec.ptr_push(externs, vec.ptr_get(new_ext, k));
                    k = k + (1 as usize);
                }
                i = ql_ptr_to_usize(vec.ptr_get(ext_result, 1));
                skip;
            }
            check (ty == tokens.REALM) {
                make realm_result: link void = parse_realm_def(toks, i);
                check (realm_result == 0) {
                    stop;
                }
                make name: str = vec.ptr_get(realm_result, 0) as str;
                make crafts: link void = vec.ptr_get(realm_result, 1);
                make item: link void = vec.ptr_new();
                vec.ptr_push(item, "realm");
                vec.ptr_push(item, name);
                vec.ptr_push(item, crafts);
                vec.ptr_push(items, item);
                i = ql_ptr_to_usize(vec.ptr_get(realm_result, 2));
                skip;
            }
            check (ty == tokens.CRAFT) {
                make fn_result: link void = parse_fn_def(toks, i);
                check (fn_result == 0) {
                    stop;
                }
                make fn_def: link void = vec.ptr_get(fn_result, 0);
                make item: link void = vec.ptr_new();
                vec.ptr_push(item, "craft");
                vec.ptr_push(item, fn_def);
                vec.ptr_push(items, item);
                i = ql_ptr_to_usize(vec.ptr_get(fn_result, 1));
                skip;
            }
            stop;
        }
        make num_items: usize = vec.ptr_len(items);
        check (num_items == 1) {
            make first: link void = vec.ptr_get(items, 0);
            make tag: str = vec.ptr_get(first, 0) as str;
            check (lexer.str_eq(tag, "craft")) {
                vec.ptr_push(result, "fn");
                vec.ptr_push(result, vec.ptr_get(first, 1));
                send result;
            }
        }
        check (num_items > 1) {
            vec.ptr_push(result, "program");
            vec.ptr_push(result, items);
            send result;
        }
        make stmts: link void = vec.ptr_new();
        make shift j: usize = i;
        loopwhile (j < n) {
            make vd: link void = parse_var_decl(toks, j);
            check (vd == 0) {
                stop;
            }
            make pair: link void = vec.ptr_get(vd, 0);
            j = ql_ptr_to_usize(vec.ptr_get(vd, 1));
            vec.ptr_push(stmts, pair);
        }
        make expr_result: link void = parse_expr(toks, j);
        check (expr_result == 0) {
            send 0;
        }
        make result_expr: link void = vec.ptr_get(expr_result, 0);
        vec.ptr_push(result, "script");
        vec.ptr_push(result, stmts);
        vec.ptr_push(result, result_expr);
        send result;
    }
}
