// QuinusLang semantic analysis for bootstrap compiler
// Type-checks AST, builds symbol table (two vecs: names, types)
// Build: quinus build compiler/semantic_test.q

bring "vec";
bring "compiler.ast";
bring "compiler.lexer";

extern craft ql_ast_expr_tag(p: link void) -> i32;
extern craft ql_ptr_to_usize(p: link void) -> usize;
extern craft ql_ast_expr_int(p: link void) -> i64;
extern craft ql_ast_expr_str(p: link void) -> str;
extern craft ql_ast_expr_left(p: link void) -> link void;
extern craft ql_ast_expr_right(p: link void) -> link void;
extern craft ql_ast_expr_args(p: link void) -> link void;
extern craft strlen(s: str) -> usize;

realm semantic {
    // Symbol table: names_vec and types_vec (parallel arrays)
    // symtab_put adds/updates; symtab_get returns type or "" if not found

    craft symtab_put(names: link void, types: link void, name: str, ty: str) -> void {
        vec.ptr_push(names, name);
        vec.ptr_push(types, ty);
        send;
    }

    // Search backwards so last definition wins
    craft symtab_get(names: link void, types: link void, name: str) -> str {
        make n: usize = vec.ptr_len(names);
        make shift i: usize = n;
        loopwhile (i > 0) {
            i = i - (1 as usize);
            make k: link void = vec.ptr_get(names, i);
            check (lexer.str_eq(k as str, name)) {
                make t: link void = vec.ptr_get(types, i);
                send t as str;
            }
        }
        send "";
    }

    // check_expr: returns type string ("i64", "str", etc.) or "" on error
    craft check_expr(expr: link void, names: link void, types: link void) -> str {
        check (expr == 0) {
            send "";
        }
        make tag: i32 = ql_ast_expr_tag(expr);
        check (tag == ast.EXPR_LITERAL) {
            send "i64";
        }
        check (tag == ast.EXPR_STR) {
            send "str";
        }
        check (tag == ast.EXPR_IDENT) {
            make name: str = ql_ast_expr_str(expr);
            make ty: str = symtab_get(names, types, name);
            send ty;
        }
        check (tag == ast.EXPR_BINARY) {
            make left: link void = ql_ast_expr_left(expr);
            make right: link void = ql_ast_expr_right(expr);
            make lt: str = check_expr(left, names, types);
            make rt: str = check_expr(right, names, types);
            check (strlen(lt) == 0 || strlen(rt) == 0) {
                send "";
            }
            check (lexer.str_eq(lt, rt)) {
                send lt;
            }
            send "";
        }
        check (tag == ast.EXPR_CALL) {
            make callee: link void = ql_ast_expr_left(expr);
            make args: link void = ql_ast_expr_args(expr);
            check (callee == 0) {
                send "";
            }
            make ct: i32 = ql_ast_expr_tag(callee);
            check (ct == ast.EXPR_IDENT) {
                send "i64";
            }
            check (ct == ast.EXPR_FIELD) {
                send "i64";
            }
            send "";
        }
        check (tag == ast.EXPR_FIELD) {
            send "i64";
        }
        send "";
    }

    // check_fn: type-check function body. Returns true if ok.
    craft check_fn(fn_def: link void) -> bool {
        check (fn_def == 0) {
            send false;
        }
        make body: link void = vec.ptr_get(fn_def, 2);
        make names: link void = vec.ptr_new();
        make types: link void = vec.ptr_new();
        make ok: bool = check_block(body, names, types);
        send ok;
    }

    // check_block: type-check stmt list
    craft check_block(stmts: link void, names: link void, types: link void) -> bool {
        check (stmts == 0) {
            send true;
        }
        make n: usize = vec.ptr_len(stmts);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make stmt: link void = vec.ptr_get(stmts, i);
            make tag_ptr: link void = vec.ptr_get(stmt, 0);
            make tag_val: i32 = ql_ptr_to_usize(tag_ptr) as i32;
            check (tag_val == 10) {
                make pair: link void = vec.ptr_get(stmt, 1);
                make vname: str = vec.ptr_get(pair, 0) as str;
                make init_expr: link void = vec.ptr_get(pair, 1);
                make it: str = check_expr(init_expr, names, types);
                check (strlen(it) == 0) {
                    send false;
                }
                symtab_put(names, types, vname, it);
            }
            check (tag_val == 12) {
                make cond: link void = vec.ptr_get(stmt, 1);
                make body: link void = vec.ptr_get(stmt, 2);
                make ct: str = check_expr(cond, names, types);
                check (strlen(ct) == 0) {
                    send false;
                }
                check (!check_block(body, names, types)) {
                    send false;
                }
            }
            check (tag_val == 13) {
                make cond: link void = vec.ptr_get(stmt, 1);
                make body: link void = vec.ptr_get(stmt, 2);
                make ct: str = check_expr(cond, names, types);
                check (strlen(ct) == 0) {
                    send false;
                }
                check (!check_block(body, names, types)) {
                    send false;
                }
            }
            check (tag_val == 14) {
                make ret_expr: link void = vec.ptr_get(stmt, 1);
                check (ret_expr != 0) {
                    make rt: str = check_expr(ret_expr, names, types);
                    check (strlen(rt) == 0) {
                        send false;
                    }
                }
            }
            check (tag_val == 15) {
                make expr: link void = vec.ptr_get(stmt, 1);
                make et: str = check_expr(expr, names, types);
                check (strlen(et) == 0) {
                    send false;
                }
            }
            i = i + (1 as usize);
        }
        send true;
    }
}
