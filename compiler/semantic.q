// QuinusLang semantic analysis for bootstrap compiler
// Type-checks AST, builds symbol table (two vecs: names, types)
// Build: quinus build compiler/semantic_test.q

bring "vec";
bring "compiler.ast";
bring "compiler.lexer";

extern craft ql_ast_expr_tag(p: link void) -> i32;
extern craft ql_ast_expr_int(p: link void) -> i64;
extern craft ql_ast_expr_str(p: link void) -> str;
extern craft ql_ast_expr_left(p: link void) -> link void;
extern craft ql_ast_expr_right(p: link void) -> link void;
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
        send "";
    }
}
