// Q++ AST types for bootstrap compiler
// Minimal subset: Literal, Ident, Binary, Call, Unary
// Uses tagged union: form with tag + payload fields

bring "compiler.tokens";

// Expr tags
realm ast {
    eternal EXPR_LITERAL: i32 = 0;
    eternal EXPR_IDENT: i32 = 1;
    eternal EXPR_BINARY: i32 = 2;
    eternal EXPR_CALL: i32 = 3;
    eternal EXPR_UNARY: i32 = 4;
    eternal EXPR_FIELD: i32 = 5;
    eternal EXPR_CAST: i32 = 6;
    eternal EXPR_STR: i32 = 7;

    // Stmt tags
    eternal STMT_VAR: i32 = 10;
    eternal STMT_ASSIGN: i32 = 11;
    eternal STMT_IF: i32 = 12;
    eternal STMT_WHILE: i32 = 13;
    eternal STMT_RETURN: i32 = 14;
    eternal STMT_EXPR: i32 = 15;
    eternal STMT_BLOCK: i32 = 16;

    // Expr node: tag + union of payloads
    // For Literal: int_val
    // For Ident: str_val (via data or we use a separate field)
    // For Binary: left, right, int_val=op
    // For Call: left=callee, args (link void = vec of Expr)
    form Expr {
        tag: i32,
        int_val: i64,
        str_val: str,
        left: link void,
        right: link void,
        args: link void
    }

    // Stmt node: tag + payload
    form Stmt {
        tag: i32,
        str_val: str,
        expr: link void,
        body: link void,
        else_body: link void
    }

    // Param: name + type
    form Param {
        name: str,
        ty_str: str
    }

    // FnDef: name, params vec, return_ty, body vec
    form FnDef {
        name: str,
        params: link void,
        return_ty: str,
        body: link void
    }

    // Program: top-level items (vec of FnDef, etc)
    form Program {
        items: link void
    }
}
