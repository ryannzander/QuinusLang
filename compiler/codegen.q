// QuinusLang codegen - walk AST, emit C
// Minimal: Literal, Ident, Binary (+, -, *, /)
// Build: cargo run -- build compiler/codegen_test.q

bring "vec";
bring "fmt";
bring "str";
bring "compiler.ast";
bring "compiler.lexer";

extern craft ql_ast_expr_tag(p: link void) -> i32;
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

    // Emit minimal C program: int main() { return <expr>; }
    // Use \\n to avoid newlines in string literals (C codegen)
    craft emit_program(expr: link void) -> str {
        make body: str = emit_expr(expr);
        make header: str = "#include <stdio.h>
int main(void) { long _r = ";
        make mid: str = str.concat(header, body);
        make end: str = "; printf(\"%ld\\n\", _r); return 0; }
";
        send str.concat(mid, end);
    }
}
