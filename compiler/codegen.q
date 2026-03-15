// QuinusLang codegen - walk AST, emit C
// Minimal: Literal, Ident, Binary (+, -, *, /)
// Build: cargo run -- build compiler/codegen_test.q

bring "vec";
bring "fmt";
bring "str";
bring "compiler.ast";
bring "compiler.lexer";

extern craft ql_ast_expr_tag(p: link void) -> i32;
extern craft ql_ptr_to_usize(p: link void) -> usize;
extern craft ql_ast_expr_int(p: link void) -> i64;
extern craft ql_ast_expr_str(p: link void) -> str;
extern craft ql_ast_expr_left(p: link void) -> link void;
extern craft ql_ast_expr_right(p: link void) -> link void;
extern craft strlen(s: str) -> usize;

realm codegen {
    craft op_to_c(op: i32) -> str {
        check (op == 26) { send "+"; }
        check (op == 27) { send "-"; }
        check (op == 28) { send "*"; }
        check (op == 29) { send "/"; }
        send "+";
    }

    craft emit_expr(expr: link void) -> str {
        check (expr == 0) {
            send "";
        }
        make tag: i32 = ql_ast_expr_tag(expr);
        check (tag == ast.EXPR_LITERAL) {
            make val: i64 = ql_ast_expr_int(expr);
            send fmt.alloc_i("%ld", val);
        }
        check (tag == ast.EXPR_IDENT) {
            make name: str = ql_ast_expr_str(expr);
            send name;
        }
        check (tag == ast.EXPR_BINARY) {
            make left: link void = ql_ast_expr_left(expr);
            make right: link void = ql_ast_expr_right(expr);
            make op: i32 = ql_ast_expr_int(expr) as i32;
            make l: str = emit_expr(left);
            make r: str = emit_expr(right);
            make op_s: str = op_to_c(op);
            make inner: str = str.concat(l, op_s);
            send str.concat(inner, r);
        }
        send "";
    }

    // Emit minimal C program: (long x = expr;)* long _r = expr; printf(...)
    craft emit_program(stmts: link void, result_expr: link void) -> str {
        make shift decls: str = "";
        make n: usize = vec.ptr_len(stmts);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make pair: link void = vec.ptr_get(stmts, i);
            make vname: str = vec.ptr_get(pair, 0) as str;
            make init_expr: link void = vec.ptr_get(pair, 1);
            make init_s: str = emit_expr(init_expr);
            make line: str = str.concat("long ", str.concat(vname, str.concat(" = ", str.concat(init_s, "; "))));
            decls = str.concat(decls, line);
            i = i + (1 as usize);
        }
        make body: str = emit_expr(result_expr);
        make header: str = "#include <stdio.h>
int main(void) { ";
        make mid: str = str.concat(header, decls);
        make assign: str = str.concat("long _r = ", str.concat(body, "; "));
        make end: str = str.concat(assign, "printf(\"%ld\\n\", _r); return 0; }
");
        send str.concat(mid, end);
    }

    // emit_fn_program: emit C for craft main() -> void { body }
    craft emit_fn_program(fn_def: link void) -> str {
        make name: str = vec.ptr_get(fn_def, 0) as str;
        make ret_ty: str = vec.ptr_get(fn_def, 1) as str;
        make body: link void = vec.ptr_get(fn_def, 2);
        make shift ret_c: str = "void";
        check (lexer.str_eq(ret_ty, "i64")) {
            ret_c = "long";
        }
        make body_c: str = emit_block(body);
        make header: str = "#include <stdio.h>
";
        make sig: str = str.concat(ret_c, str.concat(" ", str.concat(name, "(void) { ")));
        make end: str = str.concat(body_c, " }
");
        send str.concat(header, str.concat(sig, end));
    }

    craft emit_block(stmts: link void) -> str {
        check (stmts == 0) {
            send "";
        }
        make shift out: str = "";
        make n: usize = vec.ptr_len(stmts);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make stmt: link void = vec.ptr_get(stmts, i);
            make tag_ptr: link void = vec.ptr_get(stmt, 0);
            make tag: i32 = ql_ptr_to_usize(tag_ptr) as i32;
            check (tag == 10) {
                make pair: link void = vec.ptr_get(stmt, 1);
                make vname: str = vec.ptr_get(pair, 0) as str;
                make init_expr: link void = vec.ptr_get(pair, 1);
                make init_s: str = emit_expr(init_expr);
                make line: str = str.concat("long ", str.concat(vname, str.concat(" = ", str.concat(init_s, "; "))));
                out = str.concat(out, line);
            }
            check (tag == 12) {
                make cond: link void = vec.ptr_get(stmt, 1);
                make inner: link void = vec.ptr_get(stmt, 2);
                make cond_s: str = emit_expr(cond);
                make body_s: str = emit_block(inner);
                make line: str = str.concat("if (", str.concat(cond_s, str.concat(") { ", str.concat(body_s, " } "))));
                out = str.concat(out, line);
            }
            check (tag == 13) {
                make cond: link void = vec.ptr_get(stmt, 1);
                make inner: link void = vec.ptr_get(stmt, 2);
                make cond_s: str = emit_expr(cond);
                make body_s: str = emit_block(inner);
                make line: str = str.concat("while (", str.concat(cond_s, str.concat(") { ", str.concat(body_s, " } "))));
                out = str.concat(out, line);
            }
            check (tag == 14) {
                make ret_expr: link void = vec.ptr_get(stmt, 1);
                make shift line: str = "return; ";
                check (ret_expr != 0) {
                    make ret_s: str = emit_expr(ret_expr);
                    line = str.concat("return ", str.concat(ret_s, "; "));
                }
                out = str.concat(out, line);
            }
            check (tag == 15) {
                make expr: link void = vec.ptr_get(stmt, 1);
                make line: str = str.concat(emit_expr(expr), "; ");
                out = str.concat(out, line);
            }
            i = i + (1 as usize);
        }
        send out;
    }
}
