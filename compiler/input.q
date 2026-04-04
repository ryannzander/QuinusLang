// Q++ standard library: File system operations
// Uses C FFI to wrap fopen, fread, fwrite, fclose

extern craft fopen(path: str, mode: str) -> link void;
extern craft fclose(stream: link void) -> i32;
extern craft fread(buf: link void, size: usize, n: usize, stream: link void) -> usize;
extern craft fwrite(buf: link void, size: usize, n: usize, stream: link void) -> usize;
extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;
extern craft fseek(stream: link void, offset: i64, whence: i32) -> i32;
extern craft ftell(stream: link void) -> i64;

realm fs {
    craft open_file(path: str, mode: str) -> link void {
        send fopen(path, mode);
    }

    craft close(stream: link void) -> i32 {
        send fclose(stream);
    }

    craft read_all(stream: link void) -> str {
        fseek(stream, 0, 2);
        make size: i64 = ftell(stream);
        fseek(stream, 0, 0);
        check (size <= 0) {
            send "";
        }
        make cap: int = (size as int) + 1;
        make buf: link void = malloc(cap as usize);
        fread(buf, 1, size as usize, stream);
        __ql_null_at(buf, size as usize);
        send buf;
    }

    craft exists(path: str) -> bool {
        make f: link void = fopen(path, "r");
        check (f == 0) {
            send false;
        }
        fclose(f);
        send true;
    }

    craft write_all(path: str, content: str) -> i32 {
        make f: link void = fopen(path, "w");
        check (f == 0) {
            send -1;
        }
        make n: usize = strlen(content);
        make written: usize = fwrite(content, 1, n, f);
        fclose(f);
        send (written == n) as i32;
    }
}
// Q++ standard library: Growable arrays
// VecPtr: array of void* (for AST nodes, tokens)
// VecU8: growable byte buffer (for string building)

extern craft malloc(size: usize) -> link void;
extern craft free(ptr: link void) -> void;
extern craft realloc(ptr: link void, size: usize) -> link void;
extern craft ql_vec_ptr_new() -> link void;
extern craft ql_vec_ptr_push(v: link void, ptr: link void) -> void;
extern craft ql_vec_ptr_get(v: link void, i: usize) -> link void;
extern craft ql_vec_ptr_len(v: link void) -> usize;
extern craft ql_vec_ptr_clear(v: link void) -> void;
extern craft ql_vec_ptr_free(v: link void) -> void;
extern craft ql_vec_u8_new() -> link void;
extern craft ql_vec_u8_push(v: link void, b: u8) -> void;
extern craft ql_vec_u8_append(v: link void, s: str) -> void;
extern craft ql_vec_u8_len(v: link void) -> usize;
extern craft ql_vec_u8_to_str(v: link void) -> str;
extern craft ql_vec_u8_clear(v: link void) -> void;
extern craft ql_vec_u8_free(v: link void) -> void;

realm vec {
    craft ptr_new() -> link void {
        send ql_vec_ptr_new();
    }

    craft ptr_push(v: link void, ptr: link void) -> void {
        ql_vec_ptr_push(v, ptr);
        send;
    }

    craft ptr_get(v: link void, i: usize) -> link void {
        send ql_vec_ptr_get(v, i);
    }

    craft ptr_len(v: link void) -> usize {
        send ql_vec_ptr_len(v);
    }

    craft ptr_clear(v: link void) -> void {
        ql_vec_ptr_clear(v);
        send;
    }

    craft ptr_free(v: link void) -> void {
        ql_vec_ptr_free(v);
        send;
    }

    craft u8_new() -> link void {
        send ql_vec_u8_new();
    }

    craft u8_push(v: link void, b: u8) -> void {
        ql_vec_u8_push(v, b);
        send;
    }

    craft u8_append(v: link void, s: str) -> void {
        ql_vec_u8_append(v, s);
        send;
    }

    craft u8_len(v: link void) -> usize {
        send ql_vec_u8_len(v);
    }

    craft u8_to_str(v: link void) -> str {
        send ql_vec_u8_to_str(v);
    }

    craft u8_clear(v: link void) -> void {
        ql_vec_u8_clear(v);
        send;
    }

    craft u8_free(v: link void) -> void {
        ql_vec_u8_free(v);
        send;
    }
}
// Token type constants for Q++ lexer
// Matches Rust lexer Token enum order for bootstrap compatibility

realm tokens {
    eternal CRAFT: i32 = 0;
    eternal MAKE: i32 = 1;
    eternal SEND: i32 = 2;
    eternal CHECK: i32 = 3;
    eternal OTHERWISE: i32 = 4;
    eternal FOR: i32 = 5;
    eternal LOOPWHILE: i32 = 6;
    eternal FOREACH: i32 = 7;
    eternal IN: i32 = 8;
    eternal FORM: i32 = 9;
    eternal STATE: i32 = 10;
    eternal BRING: i32 = 11;
    eternal LPAREN: i32 = 12;
    eternal RPAREN: i32 = 13;
    eternal LBRACE: i32 = 14;
    eternal RBRACE: i32 = 15;
    eternal SEMICOLON: i32 = 16;
    eternal COMMA: i32 = 17;
    eternal COLON: i32 = 18;
    eternal EQ: i32 = 19;
    eternal EQEQ: i32 = 20;
    eternal NE: i32 = 21;
    eternal LT: i32 = 22;
    eternal LE: i32 = 23;
    eternal GT: i32 = 24;
    eternal GE: i32 = 25;
    eternal PLUS: i32 = 26;
    eternal MINUS: i32 = 27;
    eternal STAR: i32 = 28;
    eternal SLASH: i32 = 29;
    eternal IDENT: i32 = 30;
    eternal INT: i32 = 31;
    eternal STR: i32 = 32;
    eternal BOOL: i32 = 33;
    eternal EOF: i32 = 34;
    eternal ARROW: i32 = 35;
    eternal DOT: i32 = 36;
    eternal EXTERN: i32 = 37;
    eternal REALM: i32 = 38;
    eternal LINK: i32 = 39;
    eternal ANDAND: i32 = 40;
    eternal OROR: i32 = 41;
}
// Q++ lexer - hand-written tokenizer for bootstrap compiler
// Output: vec of tokens (link void = token pointer)
// Use: bring "compiler.lexer"; toks = lexer.tokenize(source);


extern craft ql_token_create(ty: i32, line: usize, col: usize, str_val: str, int_val: i64) -> link void;
extern craft ql_token_ty(t: link void) -> i32;
extern craft ql_token_str(t: link void) -> str;
extern craft ql_token_int(t: link void) -> i64;
extern craft ql_str_at(s: str, i: usize) -> i32;
extern craft ql_str_sub(s: str, start: usize, end: usize) -> str;
extern craft strlen(s: str) -> usize;

realm lexer {
    craft token_ty(t: link void) -> i32 {
        send ql_token_ty(t);
    }

    craft token_str(t: link void) -> str {
        send ql_token_str(t);
    }

    craft token_int(t: link void) -> i64 {
        send ql_token_int(t);
    }

    craft tokenize(source: str) -> link void {
        make tok_list: link void = vec.ptr_new();
        check (source == 0) {
            vec.ptr_push(tok_list, ql_token_create(tokens.EOF, 1, 1, "", 0));
            send tok_list;
        }
        make n: usize = strlen(source);
        make shift i: usize = 0;
        check (n >= 3 && ql_str_at(source, 0) == 239 && ql_str_at(source, 1) == 187 && ql_str_at(source, 2) == 191) {
            i = 3 as usize;
        }
        make shift line: usize = 1;
        make shift col: usize = 1;
        loopwhile (i < n) {
            make c: i32 = ql_str_at(source, i);
            check (c < 0) {
                stop;
            }
            check (c == 32 || c == 9 || c == 13) {
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 10) {
                i = i + (1 as usize);
                line = line + (1 as usize);
                col = (1 as usize);
                skip;
            }
            check (c == 47 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 47) {
                i = i + (2 as usize);
                loopwhile (i < n && ql_str_at(source, i) != 10) {
                    i = i + (1 as usize);
                }
                skip;
            }
            check (c == 40) {
                vec.ptr_push(tok_list, ql_token_create(tokens.LPAREN, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 41) {
                vec.ptr_push(tok_list, ql_token_create(tokens.RPAREN, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 123) {
                vec.ptr_push(tok_list, ql_token_create(tokens.LBRACE, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 125) {
                vec.ptr_push(tok_list, ql_token_create(tokens.RBRACE, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 59) {
                vec.ptr_push(tok_list, ql_token_create(tokens.SEMICOLON, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 44) {
                vec.ptr_push(tok_list, ql_token_create(tokens.COMMA, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 58) {
                vec.ptr_push(tok_list, ql_token_create(tokens.COLON, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 43) {
                vec.ptr_push(tok_list, ql_token_create(tokens.PLUS, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 45 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 62) {
                vec.ptr_push(tok_list, ql_token_create(tokens.ARROW, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 45) {
                vec.ptr_push(tok_list, ql_token_create(tokens.MINUS, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 42) {
                vec.ptr_push(tok_list, ql_token_create(tokens.STAR, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 47) {
                vec.ptr_push(tok_list, ql_token_create(tokens.SLASH, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 60 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 61) {
                vec.ptr_push(tok_list, ql_token_create(tokens.LE, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 60) {
                vec.ptr_push(tok_list, ql_token_create(tokens.LT, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 62 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 61) {
                vec.ptr_push(tok_list, ql_token_create(tokens.GE, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 62) {
                vec.ptr_push(tok_list, ql_token_create(tokens.GT, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 33 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 61) {
                vec.ptr_push(tok_list, ql_token_create(tokens.NE, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 61 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 61) {
                vec.ptr_push(tok_list, ql_token_create(tokens.EQEQ, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 61) {
                vec.ptr_push(tok_list, ql_token_create(tokens.EQ, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 38 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 38) {
                vec.ptr_push(tok_list, ql_token_create(tokens.ANDAND, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 124 && (i + (1 as usize)) < n && ql_str_at(source, i + (1 as usize)) == 124) {
                vec.ptr_push(tok_list, ql_token_create(tokens.OROR, line, col, "", 0));
                i = i + (2 as usize);
                col = col + (2 as usize);
                skip;
            }
            check (c == 46) {
                vec.ptr_push(tok_list, ql_token_create(tokens.DOT, line, col, "", 0));
                i = i + (1 as usize);
                col = col + (1 as usize);
                skip;
            }
            check (c == 34) {
                make start: usize = i + (1 as usize);
                i = i + (1 as usize);
                make shift found: bool = false;
                loopwhile (i < n) {
                    make cc: i32 = ql_str_at(source, i);
                    check (cc == 34) {
                        i = i + (1 as usize);
                        make lit: str = ql_str_sub(source, start, i - (1 as usize));
                        vec.ptr_push(tok_list, ql_token_create(tokens.STR, line, col, lit, 0));
                        col = col + (i - start) + (2 as usize);
                        found = true;
                        stop;
                    }
                    check (cc == 92) {
                        i = i + (2 as usize);
                        skip;
                    }
                    i = i + (1 as usize);
                }
                check (!found) {
                    vec.ptr_push(tok_list, ql_token_create(tokens.STR, line, col, "", 0));
                }
                skip;
            }
            check (c >= 48 && c <= 57) {
                make start: usize = i;
                loopwhile (i < n) {
                    make cc: i32 = ql_str_at(source, i);
                    check (cc < 48 || cc > 57) {
                        stop;
                    }
                    i = i + (1 as usize);
                }
                make lit: str = ql_str_sub(source, start, i);
                make shift val: i64 = 0;
                make shift j: usize = 0;
                loopwhile (j < strlen(lit)) {
                    make d: i32 = ql_str_at(lit, j) - 48;
                    val = val * (10 as i64) + (d as i64);
                    j = j + (1 as usize);
                }
                vec.ptr_push(tok_list, ql_token_create(tokens.INT, line, col, "", val));
                col = col + (i - start);
                skip;
            }
            check ((c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c == 95) {
                make start: usize = i;
                loopwhile (i < n) {
                    make cc: i32 = ql_str_at(source, i);
                    check ((cc >= 48 && cc <= 57) || (cc >= 65 && cc <= 90) || (cc >= 97 && cc <= 122) || cc == 95) {
                        i = i + (1 as usize);
                        skip;
                    }
                    stop;
                }
                make ident: str = ql_str_sub(source, start, i);
                make kw: i32 = keyword_type(ident);
                check (kw >= 0) {
                    vec.ptr_push(tok_list, ql_token_create(kw, line, col, "", 0));
                }
                otherwise {
                    vec.ptr_push(tok_list, ql_token_create(tokens.IDENT, line, col, ident, 0));
                }
                col = col + (i - start);
                skip;
            }
            i = i + (1 as usize);
            col = col + (1 as usize);
        }
        vec.ptr_push(tok_list, ql_token_create(tokens.EOF, line, col, "", 0));
        send tok_list;
    }

    craft keyword_type(s: str) -> i32 {
        check (str_eq(s, "craft")) { send tokens.CRAFT; }
        check (str_eq(s, "make")) { send tokens.MAKE; }
        check (str_eq(s, "send")) { send tokens.SEND; }
        check (str_eq(s, "check")) { send tokens.CHECK; }
        check (str_eq(s, "otherwise")) { send tokens.OTHERWISE; }
        check (str_eq(s, "for")) { send tokens.FOR; }
        check (str_eq(s, "loopwhile")) { send tokens.LOOPWHILE; }
        check (str_eq(s, "foreach")) { send tokens.FOREACH; }
        check (str_eq(s, "in")) { send tokens.IN; }
        check (str_eq(s, "form")) { send tokens.FORM; }
        check (str_eq(s, "state")) { send tokens.STATE; }
        check (str_eq(s, "bring")) { send tokens.BRING; }
        check (str_eq(s, "extern")) { send tokens.EXTERN; }
        check (str_eq(s, "realm")) { send tokens.REALM; }
        check (str_eq(s, "link")) { send tokens.LINK; }
        check (str_eq(s, "true")) { send tokens.BOOL; }
        check (str_eq(s, "false")) { send tokens.BOOL; }
        send -1;
    }

    craft str_eq(a: str, b: str) -> bool {
        check (a == 0 && b == 0) { send true; }
        check (a == 0 || b == 0) { send false; }
        make na: usize = strlen(a);
        make nb: usize = strlen(b);
        check (na != nb) { send false; }
        make shift k: usize = 0;
        loopwhile (k < na) {
            check (ql_str_at(a, k) != ql_str_at(b, k)) { send false; }
            k = k + (1 as usize);
        }
        send true;
    }
}
// Q++ standard library: String utilities
// trim, concat (beyond +)

extern craft malloc(size: usize) -> link void;
extern craft ql_str_trim(s: str) -> str;
extern craft ql_str_concat(a: str, b: str) -> str;

realm str {
    craft trim(s: str) -> str {
        check (s == 0) {
            send "";
        }
        send ql_str_trim(s);
    }

    craft concat(a: str, b: str) -> str {
        check (a == 0) {
            send b;
        }
        check (b == 0) {
            send a;
        }
        send ql_str_concat(a, b);
    }
}
// Q++ AST types for bootstrap compiler
// Minimal subset: Literal, Ident, Binary, Call, Unary
// Uses tagged union: form with tag + payload fields


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
// Q++ parser - minimal recursive descent
// Reads tokens from lexer, produces AST (subset: int, ident, binary +)
// Build: qpp build compiler/parser.q


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
    // parse_expr: top level, delegates to parse_logical_or
    craft parse_expr(toks: link void, i: usize) -> link void {
        send parse_logical_or(toks, i);
    }

    // parse_logical_or: logical_and || logical_and | ...
    craft parse_logical_or(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_logical_and(toks, i);
        check (left_result == 0) {
            send 0;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx + (1 as usize) < n) {
            make tok_op: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(tok_op) != tokens.OROR) {
                stop;
            }
            make right_result: link void = parse_logical_and(toks, idx + (1 as usize));
            check (right_result == 0) {
                stop;
            }
            make right: link void = vec.ptr_get(right_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(right_result, 1));
            left = ast_helpers.new_expr_binary(left, tokens.OROR, right);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, left);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
    }

    // parse_logical_and: compare && compare | ...
    craft parse_logical_and(toks: link void, i: usize) -> link void {
        make left_result: link void = parse_compare(toks, i);
        check (left_result == 0) {
            send 0;
        }
        make shift left: link void = vec.ptr_get(left_result, 0);
        make shift idx: usize = ql_ptr_to_usize(vec.ptr_get(left_result, 1));
        make n: usize = vec.ptr_len(toks);
        loopwhile (idx + (1 as usize) < n) {
            make tok_op: link void = vec.ptr_get(toks, idx);
            check (lexer.token_ty(tok_op) != tokens.ANDAND) {
                stop;
            }
            make right_result: link void = parse_compare(toks, idx + (1 as usize));
            check (right_result == 0) {
                stop;
            }
            make right: link void = vec.ptr_get(right_result, 0);
            idx = ql_ptr_to_usize(vec.ptr_get(right_result, 1));
            left = ast_helpers.new_expr_binary(left, tokens.ANDAND, right);
        }
        make result: link void = vec.ptr_new();
        vec.ptr_push(result, left);
        vec.ptr_push(result, ql_usize_to_ptr(idx));
        send result;
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
            make shift next_idx: usize = ql_ptr_to_usize(vec.ptr_get(block_result, 0));
            make stmt: link void = vec.ptr_new();
            vec.ptr_push(stmt, ql_usize_to_ptr((ast.STMT_IF as i64) as usize));
            vec.ptr_push(stmt, cond);
            vec.ptr_push(stmt, body);
            check (next_idx + (1 as usize) < n) {
                make tok_next: link void = vec.ptr_get(toks, next_idx);
                check (lexer.token_ty(tok_next) == tokens.OTHERWISE) {
                    check (next_idx + (4 as usize) >= n) {
                        make result: link void = vec.ptr_new();
                        vec.ptr_push(result, stmt);
                        vec.ptr_push(result, ql_usize_to_ptr(next_idx));
                        send result;
                    }
                    make obrace: link void = vec.ptr_get(toks, next_idx + (1 as usize));
                    check (lexer.token_ty(obrace) != tokens.LBRACE) {
                        make result: link void = vec.ptr_new();
                        vec.ptr_push(result, stmt);
                        vec.ptr_push(result, ql_usize_to_ptr(next_idx));
                        send result;
                    }
                    make else_result: link void = parse_block(toks, next_idx + (2 as usize));
                    check (else_result != 0) {
                        make else_body: link void = vec.ptr_get(else_result, 1);
                        vec.ptr_push(stmt, else_body);
                        next_idx = ql_ptr_to_usize(vec.ptr_get(else_result, 0));
                    }
                }
            }
            make result: link void = vec.ptr_new();
            vec.ptr_push(result, stmt);
            vec.ptr_push(result, ql_usize_to_ptr(next_idx));
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
        make params: link void = vec.ptr_new();
        make shift j: usize = i + (3 as usize);
        make t_j: link void = vec.ptr_get(toks, j);
        check (lexer.token_ty(t_j) != tokens.RPAREN) {
            loopwhile (j < n) {
                make tp: link void = vec.ptr_get(toks, j);
                check (lexer.token_ty(tp) != tokens.IDENT) {
                    stop;
                }
                make pname: str = lexer.token_str(tp);
                j = j + (1 as usize);
                check (j >= n || lexer.token_ty(vec.ptr_get(toks, j)) != tokens.COLON) {
                    stop;
                }
                j = j + (1 as usize);
                make shift ty_parts: link void = vec.ptr_new();
                loopwhile (j < n) {
                    make tt: link void = vec.ptr_get(toks, j);
                    make ty_t: i32 = lexer.token_ty(tt);
                    check (ty_t == tokens.COMMA || ty_t == tokens.RPAREN) {
                        stop;
                    }
                    check (ty_t == tokens.LINK || ty_t == tokens.IDENT) {
                        vec.ptr_push(ty_parts, lexer.token_str(tt));
                        j = j + (1 as usize);
                        skip;
                    }
                    stop;
                }
                make shift pty: str = vec.ptr_get(ty_parts, 0) as str;
                check (vec.ptr_len(ty_parts) == 2) {
                    pty = str.concat(vec.ptr_get(ty_parts, 0) as str, str.concat(" ", vec.ptr_get(ty_parts, 1) as str));
                }
                make pair: link void = vec.ptr_new();
                vec.ptr_push(pair, pname);
                vec.ptr_push(pair, pty);
                vec.ptr_push(params, pair);
                check (j >= n) {
                    stop;
                }
                make t_comma: link void = vec.ptr_get(toks, j);
                check (lexer.token_ty(t_comma) != tokens.COMMA) {
                    stop;
                }
                j = j + (1 as usize);
            }
        }
        check (j >= n) {
            send 0;
        }
        make t_rp: link void = vec.ptr_get(toks, j);
        check (lexer.token_ty(t_rp) != tokens.RPAREN) {
            send 0;
        }
        j = j + (1 as usize);
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
        vec.ptr_push(fn_def, params);
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
// Q++ semantic analysis for bootstrap compiler
// Type-checks AST, builds symbol table (two vecs: names, types)
// Build: qpp build compiler/semantic_test.q


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
            make op: i32 = ql_ast_expr_int(expr) as i32;
            make lt: str = check_expr(left, names, types);
            make rt: str = check_expr(right, names, types);
            check (strlen(lt) == 0 || strlen(rt) == 0) {
                send "";
            }
            check (lexer.str_eq(lt, rt)) {
                send lt;
            }
            check (op == tokens.EQEQ || op == tokens.NE || op == tokens.LT || op == tokens.LE || op == tokens.GT || op == tokens.GE) {
                send "i64";
            }
            check (op == tokens.ANDAND || op == tokens.OROR) {
                send "i64";
            }
            check (op == tokens.PLUS || op == tokens.MINUS || op == tokens.STAR || op == tokens.SLASH) {
                check (lexer.str_eq(lt, "i64") || lexer.str_eq(lt, "int") || lexer.str_eq(lt, "usize")) {
                    check (lexer.str_eq(rt, "i64") || lexer.str_eq(rt, "int") || lexer.str_eq(rt, "usize")) {
                        send "i64";
                    }
                }
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
        check (tag == ast.EXPR_CAST) {
            make inner: link void = ql_ast_expr_left(expr);
            make inner_ty: str = check_expr(inner, names, types);
            check (strlen(inner_ty) == 0) {
                send "";
            }
            make target: str = ql_ast_expr_str(expr);
            send target;
        }
        send "";
    }

    // check_fn: type-check function body. Returns true if ok.
    craft check_fn(fn_def: link void) -> bool {
        check (fn_def == 0) {
            send false;
        }
        make params: link void = vec.ptr_get(fn_def, 2);
        make body: link void = vec.ptr_get(fn_def, 3);
        make names: link void = vec.ptr_new();
        make types: link void = vec.ptr_new();
        make shift np: usize = 0;
        check (params != 0) {
            np = vec.ptr_len(params);
        }
        make shift i: usize = 0;
        loopwhile (i < np) {
            make pair: link void = vec.ptr_get(params, i);
            make pname: str = vec.ptr_get(pair, 0) as str;
            make pty: str = vec.ptr_get(pair, 1) as str;
            symtab_put(names, types, pname, pty);
            i = i + (1 as usize);
        }
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
                check (vec.ptr_len(stmt) >= 4) {
                    make else_body: link void = vec.ptr_get(stmt, 3);
                    check (!check_block(else_body, names, types)) {
                        send false;
                    }
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
// Q++ standard library: String formatting (sprintf-style)
// For codegen string building

extern craft malloc(size: usize) -> link void;
extern craft snprintf(buf: str, size: usize, fmt: str, a: i64) -> i32;
extern craft ql_fmt_sprintf_s(buf: str, size: usize, fmt: str, s: str) -> i32;
extern craft ql_fmt_sprintf_ii(buf: str, size: usize, fmt: str, a: i64, b: i64) -> i32;
extern craft ql_fmt_sprintf_si(buf: str, size: usize, fmt: str, s: str, a: i64) -> i32;
extern craft ql_fmt_sprintf_ss(buf: str, size: usize, fmt: str, a: str, b: str) -> i32;
extern craft ql_fmt_alloc_i(fmt: str, a: i64) -> str;
extern craft ql_fmt_alloc_s(fmt: str, s: str) -> str;
extern craft ql_fmt_alloc_si(fmt: str, s: str, a: i64) -> str;

realm fmt {
    craft sprintf_i(buf: str, size: usize, fmt: str, a: i64) -> i32 {
        send snprintf(buf, size, fmt, a);
    }

    craft sprintf_s(buf: str, size: usize, fmt: str, s: str) -> i32 {
        send ql_fmt_sprintf_s(buf, size, fmt, s);
    }

    craft sprintf_ii(buf: str, size: usize, fmt: str, a: i64, b: i64) -> i32 {
        send ql_fmt_sprintf_ii(buf, size, fmt, a, b);
    }

    craft sprintf_si(buf: str, size: usize, fmt: str, s: str, a: i64) -> i32 {
        send ql_fmt_sprintf_si(buf, size, fmt, s, a);
    }

    craft sprintf_ss(buf: str, size: usize, fmt: str, a: str, b: str) -> i32 {
        send ql_fmt_sprintf_ss(buf, size, fmt, a, b);
    }

    craft alloc_i(fmt: str, a: i64) -> str {
        send ql_fmt_alloc_i(fmt, a);
    }

    craft alloc_s(fmt: str, s: str) -> str {
        send ql_fmt_alloc_s(fmt, s);
    }

    craft alloc_si(fmt: str, s: str, a: i64) -> str {
        send ql_fmt_alloc_si(fmt, s, a);
    }
}
// C runtime for ql_* functions - emitted by codegen when building self-contained output
// Bring "compiler.runtime" from codegen


realm runtime {
    craft emit() -> str {
        make str_rt: str = "static char* ql_str_trim(const char* s) {
    if (!s) return (char*)\"\";
    while (*s == ' ' || *s == '\\t' || *s == '\\n' || *s == '\\r') s++;
    const char* end = s;
    while (*end) end++;
    while (end > s && (end[-1] == ' ' || end[-1] == '\\t' || end[-1] == '\\n' || end[-1] == '\\r')) end--;
    size_t n = end - s;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s, n);
    r[n] = 0;
    return r;
}
static char* ql_str_concat(const char* a, const char* b) {
    if (!a) a = \"\";
    if (!b) b = \"\";
    size_t la = strlen(a), lb = strlen(b);
    char* r = (char*)malloc(la + lb + 1);
    memcpy(r, a, la + 1);
    strcat(r, b);
    return r;
}
";
        make vec_rt: str = "typedef struct { void** data; size_t len; size_t cap; } ql_vec_ptr_t;
typedef struct { char* data; size_t len; size_t cap; } ql_vec_u8_t;
static void* ql_vec_ptr_new(void) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)malloc(sizeof(ql_vec_ptr_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
static void ql_vec_ptr_push(void* vp, void* ptr) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 16;
        v->data = (void**)realloc(v->data, ncap * sizeof(void*));
        v->cap = ncap;
    }
    v->data[v->len++] = ptr;
}
static void* ql_vec_ptr_get(void* vp, size_t i) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    return (i < v->len) ? v->data[i] : 0;
}
static size_t ql_vec_ptr_len(void* vp) { return ((ql_vec_ptr_t*)vp)->len; }
static void ql_vec_ptr_clear(void* vp) { ((ql_vec_ptr_t*)vp)->len = 0; }
static void ql_vec_ptr_free(void* vp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    free(v->data);
    free(v);
}
static void* ql_vec_u8_new(void) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)malloc(sizeof(ql_vec_u8_t));
    v->data = 0; v->len = 0; v->cap = 0;
    return v;
}
static void ql_vec_u8_push(void* vp, unsigned char b) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    if (v->len >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 64;
        v->data = (char*)realloc(v->data, ncap);
        v->cap = ncap;
    }
    v->data[v->len++] = (char)b;
}
static void ql_vec_u8_append(void* vp, const char* s) {
    if (!s) return;
    size_t n = strlen(s);
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    while (v->len + n >= v->cap) {
        size_t ncap = v->cap ? v->cap * 2 : 64;
        if (ncap < v->len + n + 1) ncap = v->len + n + 1;
        v->data = (char*)realloc(v->data, ncap);
        v->cap = ncap;
    }
    memcpy(v->data + v->len, s, n);
    v->len += n;
}
static size_t ql_vec_u8_len(void* vp) { return ((ql_vec_u8_t*)vp)->len; }
static char* ql_vec_u8_to_str(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    char* r = (char*)malloc(v->len + 1);
    memcpy(r, v->data, v->len);
    r[v->len] = 0;
    return r;
}
static void ql_vec_u8_clear(void* vp) { ((ql_vec_u8_t*)vp)->len = 0; }
static void ql_vec_u8_free(void* vp) {
    ql_vec_u8_t* v = (ql_vec_u8_t*)vp;
    free(v->data);
    free(v);
}
";
        make map_rt: str = "typedef struct { char* key; void* value; } ql_map_pair_t;
static void* ql_map_str_ptr_new(void) { return ql_vec_ptr_new(); }
static void ql_map_str_ptr_put(void* mp, const char* key, void* value) {
    void** vp = (void**)mp;
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)vp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p && strcmp(p->key, key) == 0) {
            free(p->key);
            p->value = value;
            return;
        }
    }
    ql_map_pair_t* p = (ql_map_pair_t*)malloc(sizeof(ql_map_pair_t));
    p->key = key ? strdup(key) : 0;
    p->value = value;
    ql_vec_ptr_push(mp, p);
}
static void* ql_map_str_ptr_get(void* mp, const char* key) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p && p->key && key && strcmp(p->key, key) == 0)
            return p->value;
    }
    return 0;
}
static int ql_map_str_ptr_has(void* mp, const char* key) {
    return ql_map_str_ptr_get(mp, key) != 0;
}
static size_t ql_map_str_ptr_len(void* mp) {
    return ql_vec_ptr_len(mp);
}
static void ql_map_str_ptr_free(void* mp) {
    ql_vec_ptr_t* v = (ql_vec_ptr_t*)mp;
    size_t i;
    for (i = 0; i < v->len; i++) {
        ql_map_pair_t* p = (ql_map_pair_t*)v->data[i];
        if (p) { free(p->key); free(p); }
    }
    ql_vec_ptr_free(mp);
}
";
        make fmt_rt: str = "static int ql_fmt_sprintf_s(char* buf, size_t size, const char* fmt, const char* s) {
    return snprintf(buf, size, fmt, s ? s : \"\");
}
static int ql_fmt_sprintf_ii(char* buf, size_t size, const char* fmt, long a, long b) {
    return snprintf(buf, size, fmt, a, b);
}
static int ql_fmt_sprintf_si(char* buf, size_t size, const char* fmt, const char* s, long a) {
    return snprintf(buf, size, fmt, s ? s : \"\", a);
}
static int ql_fmt_sprintf_ss(char* buf, size_t size, const char* fmt, const char* a, const char* b) {
    return snprintf(buf, size, fmt, a ? a : \"\", b ? b : \"\");
}
static char* ql_fmt_alloc_i(const char* fmt, long a) {
    char buf[64];
    int n = snprintf(buf, sizeof(buf), fmt, a);
    char* r = (char*)malloc((size_t)n + 1);
    memcpy(r, buf, (size_t)n + 1);
    return r;
}
static char* ql_fmt_alloc_s(const char* fmt, const char* s) {
    size_t n = strlen(s ? s : \"\") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : \"\");
    return r;
}
static char* ql_fmt_alloc_si(const char* fmt, const char* s, long a) {
    size_t n = strlen(s ? s : \"\") + 64;
    char* r = (char*)malloc(n);
    snprintf(r, n, fmt, s ? s : \"\"\", a);
    return r;
}
";
        make lex_rt: str = "typedef struct { int ty; size_t line; size_t col; char* str_val; long int_val; } ql_token_t;
static void* ql_token_create(int ty, size_t line, size_t col, const char* str_val, long int_val) {
    ql_token_t* t = (ql_token_t*)malloc(sizeof(ql_token_t));
    t->ty = ty; t->line = line; t->col = col;
    t->str_val = str_val ? strdup(str_val) : 0;
    t->int_val = int_val;
    return t;
}
static int ql_token_ty(void* t) { return ((ql_token_t*)t)->ty; }
static size_t ql_token_line(void* t) { return ((ql_token_t*)t)->line; }
static size_t ql_token_col(void* t) { return ((ql_token_t*)t)->col; }
static char* ql_token_str(void* t) { return ((ql_token_t*)t)->str_val; }
static long ql_token_int(void* t) { return ((ql_token_t*)t)->int_val; }
static void ql_token_free(void* t) {
    ql_token_t* tok = (ql_token_t*)t;
    free(tok->str_val);
    free(tok);
}
static int ql_str_at(const char* s, size_t i) {
    if (!s || i >= strlen(s)) return -1;
    return (unsigned char)s[i];
}
static char* ql_str_sub(const char* s, size_t start, size_t end) {
    if (!s || start >= end || end > strlen(s)) return strdup(\"\");
    size_t n = end - start;
    char* r = (char*)malloc(n + 1);
    memcpy(r, s + start, n);
    r[n] = 0;
    return r;
}
static void* ql_usize_to_ptr(size_t u) { return (void*)(uintptr_t)u; }
static size_t ql_ptr_to_usize(void* p) { return (size_t)(uintptr_t)p; }
";
        make ast_rt: str = "typedef struct { int tag; long int_val; char* str_val; void* left; void* right; void* args; } ast_Expr_t;
static void* ql_ast_expr_alloc(void) {
    return malloc(sizeof(ast_Expr_t));
}
static void ql_ast_expr_set_tag(void* p, int tag) { ((ast_Expr_t*)p)->tag = tag; }
static void ql_ast_expr_set_int(void* p, long val) { ((ast_Expr_t*)p)->int_val = val; }
static void ql_ast_expr_set_str(void* p, char* s) { ((ast_Expr_t*)p)->str_val = s; }
static void ql_ast_expr_set_left(void* p, void* left) { ((ast_Expr_t*)p)->left = left; }
static void ql_ast_expr_set_right(void* p, void* right) { ((ast_Expr_t*)p)->right = right; }
static void ql_ast_expr_set_args(void* p, void* args) { ((ast_Expr_t*)p)->args = args; }
static int ql_ast_expr_tag(void* p) { return ((ast_Expr_t*)p)->tag; }
static long ql_ast_expr_int(void* p) { return ((ast_Expr_t*)p)->int_val; }
static char* ql_ast_expr_str(void* p) { return ((ast_Expr_t*)p)->str_val; }
static void* ql_ast_expr_left(void* p) { return ((ast_Expr_t*)p)->left; }
static void* ql_ast_expr_right(void* p) { return ((ast_Expr_t*)p)->right; }
static void* ql_ast_expr_args(void* p) { return ((ast_Expr_t*)p)->args; }
";
        make r1: str = str.concat(str_rt, vec_rt);
        make r2: str = str.concat(r1, map_rt);
        make r3: str = str.concat(r2, fmt_rt);
        make r4: str = str.concat(r3, lex_rt);
        send str.concat(r4, ast_rt);
    }
}
// Q++ codegen - walk AST, emit C
// Minimal: Literal, Ident, Binary (+, -, *, /)
// Build: cargo run -- build compiler/codegen_test.q


extern craft ql_ast_expr_tag(p: link void) -> i32;
extern craft ql_ptr_to_usize(p: link void) -> usize;
extern craft ql_ast_expr_int(p: link void) -> i64;
extern craft ql_ast_expr_str(p: link void) -> str;
extern craft ql_ast_expr_left(p: link void) -> link void;
extern craft ql_ast_expr_right(p: link void) -> link void;
extern craft ql_ast_expr_args(p: link void) -> link void;
extern craft ql_ptr_to_usize(p: link void) -> usize;
extern craft strlen(s: str) -> usize;
extern craft ql_str_at(s: str, i: usize) -> i32;

realm codegen {
    craft op_to_c(op: i32) -> str {
        check (op == 26) { send "+"; }
        check (op == 27) { send "-"; }
        check (op == 28) { send "*"; }
        check (op == 29) { send "/"; }
        check (op == 20) { send "=="; }
        check (op == 21) { send "!="; }
        check (op == 22) { send "<"; }
        check (op == 23) { send "<="; }
        check (op == 24) { send ">"; }
        check (op == 25) { send ">="; }
        check (op == 40) { send "&&"; }
        check (op == 41) { send "||"; }
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
        check (tag == ast.EXPR_STR) {
            make s: str = ql_ast_expr_str(expr);
            send str.concat("\"", str.concat(s, "\""));
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
        check (tag == ast.EXPR_CALL) {
            make callee: link void = ql_ast_expr_left(expr);
            make args: link void = ql_ast_expr_args(expr);
            check (ql_ast_expr_tag(callee) == ast.EXPR_IDENT && lexer.str_eq(ql_ast_expr_str(callee), "__ql_null_at")) {
                check (vec.ptr_len(args) >= 2) {
                    make buf_s: str = emit_expr(vec.ptr_get(args, 0));
                    make pos_s: str = emit_expr(vec.ptr_get(args, 1));
                    make suffix: str = "] = 0)";
                    send str.concat("((char*)(", str.concat(buf_s, str.concat("))[", str.concat(pos_s, suffix))));
                }
            }
            make callee_s: str = emit_callee(callee);
            make args_s: str = emit_call_args(args);
            send str.concat(callee_s, str.concat("(", str.concat(args_s, ")")));
        }
        check (tag == ast.EXPR_FIELD) {
            make base: link void = ql_ast_expr_left(expr);
            make field: str = ql_ast_expr_str(expr);
            make base_s: str = emit_expr(base);
            send str.concat(base_s, str.concat("_", field));
        }
        check (tag == ast.EXPR_CAST) {
            make inner: link void = ql_ast_expr_left(expr);
            make target: str = ql_ast_expr_str(expr);
            make inner_s: str = emit_expr(inner);
            check (lexer.str_eq(target, "str")) {
                send str.concat("(char*)", inner_s);
            }
            check (lexer.str_eq(target, "usize")) {
                send str.concat("(size_t)", inner_s);
            }
            check (lexer.str_eq(target, "i32")) {
                send str.concat("(int)", inner_s);
            }
            check (lexer.str_eq(target, "link void")) {
                send str.concat("(void*)", inner_s);
            }
            send str.concat(str.concat("(", str.concat(target, ")")), inner_s);
        }
        send "";
    }

    craft emit_callee(callee: link void) -> str {
        check (callee == 0) {
            send "";
        }
        make tag: i32 = ql_ast_expr_tag(callee);
        check (tag == ast.EXPR_IDENT) {
            send ql_ast_expr_str(callee);
        }
        check (tag == ast.EXPR_FIELD) {
            make base: link void = ql_ast_expr_left(callee);
            make field: str = ql_ast_expr_str(callee);
            make base_s: str = emit_expr(base);
            send str.concat(base_s, str.concat("_", field));
        }
        send "";
    }

    craft emit_call_args(args: link void) -> str {
        check (args == 0) {
            send "";
        }
        make n: usize = vec.ptr_len(args);
        make shift out: str = "";
        make shift i: usize = 0;
        loopwhile (i < n) {
            make arg: link void = vec.ptr_get(args, i);
            make s: str = emit_expr(arg);
            check (i > 0) {
                out = str.concat(out, ", ");
            }
            out = str.concat(out, s);
            i = i + (1 as usize);
        }
        send out;
    }

    craft name_is_ql_runtime(name: str) -> bool {
        check (strlen(name) < 3) { send false; }
        check (ql_str_at(name, 0) != 113) { send false; }
        check (ql_str_at(name, 1) != 108) { send false; }
        check (ql_str_at(name, 2) != 95) { send false; }
        send true;
    }

    craft emit_externs(externs: link void) -> str {
        send emit_externs_filtered(externs, false);
    }

    craft emit_externs_filtered(externs: link void, skip_runtime: bool) -> str {
        check (externs == 0) {
            send "";
        }
        make shift out: str = "";
        make n: usize = vec.ptr_len(externs);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make ext: link void = vec.ptr_get(externs, i);
            make name: str = vec.ptr_get(ext, 0) as str;
            check (skip_runtime && name_is_ql_runtime(name)) {
                i = i + (1 as usize);
                skip;
            }
            make ret_ty: str = vec.ptr_get(ext, 1) as str;
            make shift c_ret: str = "void";
            check (lexer.str_eq(ret_ty, "usize")) {
                c_ret = "size_t";
            }
            check (lexer.str_eq(ret_ty, "i32")) {
                c_ret = "int";
            }
            check (lexer.str_eq(ret_ty, "i64")) {
                c_ret = "long";
            }
            check (lexer.str_eq(ret_ty, "str")) {
                c_ret = "char*";
            }
            check (lexer.str_eq(ret_ty, "link void")) {
                c_ret = "void*";
            }
            make line: str = str.concat("extern ", str.concat(c_ret, str.concat(" ", str.concat(name, "(); "))));
            out = str.concat(out, line);
            i = i + (1 as usize);
        }
        send out;
    }

    // Emit minimal C program: externs (long x = expr;)* long _r = expr; printf(...)
    craft emit_program(externs: link void, stmts: link void, result_expr: link void) -> str {
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
        make ext_s: str = emit_externs(externs);
        make header: str = "#include <stdio.h>
";
        make header2: str = str.concat(header, str.concat(ext_s, "int main(void) { "));
        make mid: str = str.concat(header2, decls);
        make assign: str = str.concat("long _r = ", str.concat(body, "; "));
        make end: str = str.concat(assign, "printf(\"%ld\\n\", _r); return 0; }
");
        send str.concat(mid, end);
    }

    // emit_fn_with_prefix: emit C function with optional realm prefix (prefix_name)
    craft emit_fn_with_prefix(externs: link void, fn_def: link void, prefix: str) -> str {
        make name: str = vec.ptr_get(fn_def, 0) as str;
        make shift c_name: str = name;
        check (strlen(prefix) > 0) {
            c_name = str.concat(prefix, str.concat("_", name));
        }
        make ret_ty: str = vec.ptr_get(fn_def, 1) as str;
        make params: link void = vec.ptr_get(fn_def, 2);
        make body: link void = vec.ptr_get(fn_def, 3);
        make shift ret_c: str = "void";
        check (lexer.str_eq(ret_ty, "i64")) {
            ret_c = "long";
        }
        check (lexer.str_eq(ret_ty, "usize")) {
            ret_c = "size_t";
        }
        check (lexer.str_eq(ret_ty, "i32")) {
            ret_c = "int";
        }
        check (lexer.str_eq(ret_ty, "str")) {
            ret_c = "char*";
        }
        check (lexer.str_eq(ret_ty, "link void")) {
            ret_c = "void*";
        }
        check (lexer.str_eq(ret_ty, "bool")) {
            ret_c = "int";
        }
        make params_s: str = emit_params(params);
        make body_c: str = emit_block(body);
        make sig: str = str.concat(ret_c, str.concat(" ", str.concat(c_name, str.concat(params_s, " { "))));
        make end: str = str.concat(body_c, " }
");
        send str.concat(sig, end);
    }

    craft emit_params(params: link void) -> str {
        check (params == 0) {
            send "()";
        }
        make n: usize = vec.ptr_len(params);
        check (n == 0) {
            send "()";
        }
        make shift out: str = "(";
        make shift i: usize = 0;
        loopwhile (i < n) {
            make pair: link void = vec.ptr_get(params, i);
            make pname: str = vec.ptr_get(pair, 0) as str;
            make pty: str = vec.ptr_get(pair, 1) as str;
            make shift cty: str = "long";
            check (lexer.str_eq(pty, "str")) {
                cty = "char*";
            }
            check (lexer.str_eq(pty, "i32")) {
                cty = "int";
            }
            check (lexer.str_eq(pty, "i64")) {
                cty = "long";
            }
            check (lexer.str_eq(pty, "usize")) {
                cty = "size_t";
            }
            check (lexer.str_eq(pty, "link void")) {
                cty = "void*";
            }
            check (lexer.str_eq(pty, "bool")) {
                cty = "int";
            }
            check (lexer.str_eq(pty, "u8")) {
                cty = "unsigned char";
            }
            out = str.concat(out, str.concat(cty, str.concat(" ", pname)));
            i = i + (1 as usize);
            check (i < n) {
                out = str.concat(out, ", ");
            }
        }
        send str.concat(out, ")");
    }

    // emit_program_full: emit C for full program (realms + crafts). Prefixes functions with realm name.
    craft emit_program_full(externs: link void, items: link void) -> str {
        make header: str = "#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
";
        make rt: str = runtime.emit();
        make ext_s: str = emit_externs_filtered(externs, true);
        make shift out: str = str.concat(header, rt);
        out = str.concat(out, ext_s);
        make n: usize = vec.ptr_len(items);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make item: link void = vec.ptr_get(items, i);
            make tag: str = vec.ptr_get(item, 0) as str;
            check (lexer.str_eq(tag, "realm")) {
                make realm_name: str = vec.ptr_get(item, 1) as str;
                check (lexer.str_eq(realm_name, "tokens")) {
                    make tok_defs: str = "static const int tokens_CRAFT=0,tokens_MAKE=1,tokens_SEND=2,tokens_CHECK=3,tokens_OTHERWISE=4,tokens_FOR=5,tokens_LOOPWHILE=6,tokens_FOREACH=7,tokens_IN=8,tokens_FORM=9,tokens_STATE=10,tokens_BRING=11,tokens_LPAREN=12,tokens_RPAREN=13,tokens_LBRACE=14,tokens_RBRACE=15,tokens_SEMICOLON=16,tokens_COMMA=17,tokens_COLON=18,tokens_EQ=19,tokens_EQEQ=20,tokens_NE=21,tokens_LT=22,tokens_LE=23,tokens_GT=24,tokens_GE=25,tokens_PLUS=26,tokens_MINUS=27,tokens_STAR=28,tokens_SLASH=29,tokens_IDENT=30,tokens_INT=31,tokens_STR=32,tokens_BOOL=33,tokens_EOF=34,tokens_ARROW=35,tokens_DOT=36,tokens_EXTERN=37,tokens_REALM=38,tokens_LINK=39,tokens_ANDAND=40,tokens_OROR=41;
";
                    out = str.concat(out, tok_defs);
                }
                check (lexer.str_eq(realm_name, "ast")) {
                    make ast_defs: str = "static const int ast_EXPR_LITERAL=0,ast_EXPR_IDENT=1,ast_EXPR_BINARY=2,ast_EXPR_CALL=3,ast_EXPR_UNARY=4,ast_EXPR_FIELD=5,ast_EXPR_CAST=6,ast_EXPR_STR=7,ast_STMT_VAR=10,ast_STMT_ASSIGN=11,ast_STMT_IF=12,ast_STMT_WHILE=13,ast_STMT_RETURN=14,ast_STMT_EXPR=15,ast_STMT_BLOCK=16;
";
                    out = str.concat(out, ast_defs);
                }
                make crafts: link void = vec.ptr_get(item, 2);
                make nc: usize = vec.ptr_len(crafts);
                make shift j: usize = 0;
                loopwhile (j < nc) {
                    make fn_def: link void = vec.ptr_get(crafts, j);
                    make fn_s: str = emit_fn_with_prefix(externs, fn_def, realm_name);
                    out = str.concat(out, str.concat(fn_s, "
"));
                    j = j + (1 as usize);
                }
            }
            check (lexer.str_eq(tag, "craft")) {
                make fn_def: link void = vec.ptr_get(item, 1);
                make fn_s: str = emit_fn_with_prefix(externs, fn_def, "");
                out = str.concat(out, str.concat(fn_s, "
"));
            }
            i = i + (1 as usize);
        }
        send out;
    }

    // emit_fn_program: emit C for craft main() -> void { body }
    craft emit_fn_program(externs: link void, fn_def: link void) -> str {
        make ext_s: str = emit_externs(externs);
        make name: str = vec.ptr_get(fn_def, 0) as str;
        make ret_ty: str = vec.ptr_get(fn_def, 1) as str;
        make body: link void = vec.ptr_get(fn_def, 3);
        make shift ret_c: str = "void";
        check (lexer.str_eq(ret_ty, "i64")) {
            ret_c = "long";
        }
        make body_c: str = emit_block(body);
        make header: str = "#include <stdio.h>
";
        make header2: str = str.concat(header, ext_s);
        make sig: str = str.concat(ret_c, str.concat(" ", str.concat(name, "(void) { ")));
        make end: str = str.concat(body_c, " }
");
        send str.concat(header2, str.concat(sig, end));
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
                make shift line: str = str.concat("if (", str.concat(cond_s, str.concat(") { ", str.concat(body_s, " } "))));
                check (vec.ptr_len(stmt) >= 4) {
                    make else_body: link void = vec.ptr_get(stmt, 3);
                    make else_s: str = emit_block(else_body);
                    line = str.concat(line, str.concat(" else { ", str.concat(else_s, " } ")));
                }
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
// Q++ preprocessor - bring flattening
// Resolves bring statements recursively, outputs flattened source
// Port of src/preprocess.rs


extern craft strlen(s: str) -> usize;
extern craft ql_str_at(s: str, i: usize) -> i32;
extern craft ql_str_sub(s: str, start: usize, end: usize) -> str;

realm preprocess {
    craft path_join(base: str, part: str) -> str {
        check (strlen(base) == 0) { send part; }
        make sep: str = "/";
        send str.concat(base, str.concat(sep, part));
    }

    // parse_bring_path: " \"compiler.lexer\" ;" or " compiler.lexer ;" -> "compiler.lexer" or ""
    craft parse_bring_path_rest(rest: str) -> str {
        make n: usize = strlen(rest);
        make shift i: usize = 0;
        loopwhile (i < n && (ql_str_at(rest, i) == 32 || ql_str_at(rest, i) == 9)) {
            i = i + (1 as usize);
        }
        check (i >= n) { send ""; }
        make shift path_str: str = "";
        check (ql_str_at(rest, i) == 34) {
            i = i + (1 as usize);
            make start: usize = i;
            loopwhile (i < n && ql_str_at(rest, i) != 34) {
                i = i + (1 as usize);
            }
            check (i >= n) { send ""; }
            path_str = ql_str_sub(rest, start, i);
        }
        otherwise {
            make start: usize = i;
            loopwhile (i < n && ql_str_at(rest, i) != 59 && ql_str_at(rest, i) != 32 && ql_str_at(rest, i) != 9 && ql_str_at(rest, i) != 10) {
                i = i + (1 as usize);
            }
            path_str = ql_str_sub(rest, start, i);
        }
        send path_str;
    }

    // extract_brings: returns vec of path strings like "compiler.lexer"
    craft extract_brings(source: str) -> link void {
        make result: link void = vec.ptr_new();
        make n: usize = strlen(source);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make line_start: usize = i;
            make shift j: usize = i;
            loopwhile (j < n && ql_str_at(source, j) != 10) {
                j = j + (1 as usize);
            }
            make line: str = ql_str_sub(source, line_start, j);
            make shift trim_i: usize = 0;
            make ln: usize = strlen(line);
            loopwhile (trim_i < ln && (ql_str_at(line, trim_i) == 32 || ql_str_at(line, trim_i) == 9)) {
                trim_i = trim_i + (1 as usize);
            }
            check (trim_i + (5 as usize) <= ln) {
                check (ql_str_at(line, trim_i) == 98 && ql_str_at(line, trim_i + (1 as usize)) == 114 && ql_str_at(line, trim_i + (2 as usize)) == 105 && ql_str_at(line, trim_i + (3 as usize)) == 110 && ql_str_at(line, trim_i + (4 as usize)) == 103) {
                    make rest: str = ql_str_sub(line, trim_i + (5 as usize), ln);
                    make path_str: str = parse_bring_path_rest(rest);
                    check (strlen(path_str) > 0) {
                        vec.ptr_push(result, path_str);
                    }
                }
            }
            i = j;
            check (i < n) {
                i = i + (1 as usize);
            }
        }
        send result;
    }

    // resolve_path: base_dir + path "compiler.lexer" -> try compiler/lexer.q, etc.
    craft resolve_path(base_dir: str, path_str: str) -> str {
        make n: usize = strlen(path_str);
        make shift i: usize = 0;
        make shift parts: link void = vec.ptr_new();
        make shift cur_start: usize = 0;
        loopwhile (i <= n) {
            check (i == n || ql_str_at(path_str, i) == 46) {
                make part: str = ql_str_sub(path_str, cur_start, i);
                check (strlen(part) > 0) {
                    vec.ptr_push(parts, part);
                }
                cur_start = i + (1 as usize);
            }
            i = i + (1 as usize);
        }
        make np: usize = vec.ptr_len(parts);
        check (np == 0) { send ""; }
        make shift rel: str = vec.ptr_get(parts, 0) as str;
        make shift k: usize = 1;
        loopwhile (k < np) {
            rel = path_join(rel, vec.ptr_get(parts, k) as str);
            k = k + (1 as usize);
        }
        make cand1: str = path_join(base_dir, str.concat(rel, ".q"));
        check (fs.exists(cand1)) { send cand1; }
        make shift last_part: str = vec.ptr_get(parts, np - (1 as usize)) as str;
        make cand2: str = path_join(path_join(base_dir, "src"), str.concat(last_part, ".q"));
        check (fs.exists(cand2)) { send cand2; }
        make cand3: str = path_join(path_join(base_dir, "stdlib"), str.concat(rel, ".q"));
        check (fs.exists(cand3)) { send cand3; }
        make cand4: str = path_join(path_join(base_dir, rel), "mod.q");
        check (fs.exists(cand4)) { send cand4; }
        make cand5: str = path_join(path_join(path_join(base_dir, "stdlib"), rel), "mod.q");
        check (fs.exists(cand5)) { send cand5; }
        send "";
    }

    // content_without_brings: remove bring lines, keep rest
    craft content_without_brings(source: str) -> str {
        make n: usize = strlen(source);
        make shift out: str = "";
        make shift i: usize = 0;
        loopwhile (i < n) {
            make line_start: usize = i;
            make shift j: usize = i;
            loopwhile (j < n && ql_str_at(source, j) != 10) {
                j = j + (1 as usize);
            }
            make line: str = ql_str_sub(source, line_start, j);
            make shift trim_i: usize = 0;
            make ln: usize = strlen(line);
            loopwhile (trim_i < ln && (ql_str_at(line, trim_i) == 32 || ql_str_at(line, trim_i) == 9)) {
                trim_i = trim_i + (1 as usize);
            }
            make shift is_bring: bool = false;
            check (trim_i + (5 as usize) <= ln) {
                check (ql_str_at(line, trim_i) == 98 && ql_str_at(line, trim_i + (1 as usize)) == 114 && ql_str_at(line, trim_i + (2 as usize)) == 105 && ql_str_at(line, trim_i + (3 as usize)) == 110 && ql_str_at(line, trim_i + (4 as usize)) == 103) {
                    make shift k: usize = trim_i + (5 as usize);
                    loopwhile (k < ln && (ql_str_at(line, k) == 32 || ql_str_at(line, k) == 9)) {
                        k = k + (1 as usize);
                    }
                    loopwhile (k < ln && ql_str_at(line, k) != 59) {
                        k = k + (1 as usize);
                    }
                    check (k < ln) {
                        is_bring = true;
                    }
                }
            }
            check (!is_bring) {
                out = str.concat(out, ql_str_sub(source, line_start, j));
                check (j < n) {
                    out = str.concat(out, "
");
                }
            }
            i = j;
            check (i < n) {
                i = i + (1 as usize);
            }
        }
        send out;
    }

    craft vec_contains(vec: link void, s: str) -> bool {
        make n: usize = vec.ptr_len(vec);
        make shift i: usize = 0;
        loopwhile (i < n) {
            check (lexer.str_eq(vec.ptr_get(vec, i) as str, s)) { send true; }
            i = i + (1 as usize);
        }
        send false;
    }

    craft trim_str(s: str) -> str {
        make n: usize = strlen(s);
        make shift start: usize = 0;
        loopwhile (start < n && (ql_str_at(s, start) == 32 || ql_str_at(s, start) == 9 || ql_str_at(s, start) == 10 || ql_str_at(s, start) == 13)) {
            start = start + (1 as usize);
        }
        make shift end: usize = n;
        loopwhile (end > start && (ql_str_at(s, end - (1 as usize)) == 32 || ql_str_at(s, end - (1 as usize)) == 9 || ql_str_at(s, end - (1 as usize)) == 10 || ql_str_at(s, end - (1 as usize)) == 13)) {
            end = end - (1 as usize);
        }
        send ql_str_sub(s, start, end);
    }

    craft flatten_inner(source: str, base_dir: str, seen: link void, output: link void) -> void {
        make brings: link void = extract_brings(source);
        make nb: usize = vec.ptr_len(brings);
        make shift bi: usize = 0;
        loopwhile (bi < nb) {
            make path_str: str = vec.ptr_get(brings, bi) as str;
            check (!vec_contains(seen, path_str)) {
                vec.ptr_push(seen, path_str);
                make file_path: str = resolve_path(base_dir, path_str);
                check (strlen(file_path) > 0) {
                    make f: link void = fs.open_file(file_path, "r");
                    check (f != 0) {
                        make sub_source: str = fs.read_all(f);
                        fs.close(f);
                        flatten_inner(sub_source, base_dir, seen, output);
                    }
                }
            }
            bi = bi + (1 as usize);
        }
        make body: str = content_without_brings(source);
        make trimmed: str = trim_str(body);
        check (strlen(trimmed) > 0) {
            make out_str: str = vec.ptr_get(output, 0) as str;
            make shift new_str: str = trimmed;
            check (strlen(out_str) > 0) {
                new_str = str.concat(out_str, str.concat("
", trimmed));
            }
            vec.ptr_clear(output);
            vec.ptr_push(output, new_str);
        }
        send;
    }

    craft flatten(source: str, base_dir: str) -> str {
        make seen: link void = vec.ptr_new();
        make output: link void = vec.ptr_new();
        vec.ptr_push(output, "");
        flatten_inner(source, base_dir, seen, output);
        send vec.ptr_get(output, 0) as str;
    }
}
// Q++ standard library: Process execution and environment
// Uses C FFI to wrap system(), getenv, getcwd

extern craft system(cmd: str) -> i32;
extern craft getenv(name: str) -> str;
extern craft getcwd(buf: str, size: usize) -> str;
extern craft malloc(size: usize) -> link void;

realm os {
    craft run(cmd: str) -> i32 {
        send system(cmd);
    }

    craft getenv(name: str) -> str {
        make p: str = getenv(name);
        check (p == 0) {
            send "";
        }
        send p;
    }

    craft cwd() -> str {
        make buf: link void = malloc(4096);
        make p: str = getcwd(buf, 4096);
        check (p == 0) {
            send "";
        }
        send p;
    }
}
// Q++ bootstrap compiler driver
// Pipeline: read source -> lex -> parse -> semantic -> codegen -> write C -> invoke cc
// Usage: build this, then run with input file (default: input.q)
// Build: cargo run -- build compiler/main.q


extern craft strlen(s: str) -> usize;

craft main() -> void {
    make path: str = "main.q";
    make base_dir: str = "..";
    make f: link void = fs.open_file(path, "r");
    check (f == 0) {
        writeln("Cannot open input file");
        send;
    }
    make src: str = fs.read_all(f);
    fs.close(f);
    make flat: str = preprocess.flatten(src, base_dir);
    make parsed: link void = parser.parse(flat);
    check (parsed == 0) {
        writeln("Parse failed");
        send;
    }
    make externs: link void = vec.ptr_get(parsed, 0);
    make kind: str = vec.ptr_get(parsed, 1) as str;
    make shift c_code: str = "";
    check (lexer.str_eq(kind, "fn")) {
        make fn_def: link void = vec.ptr_get(parsed, 2);
        make ok: bool = semantic.check_fn(fn_def);
        check (!ok) {
            writeln("Semantic check failed");
            send;
        }
        c_code = codegen.emit_fn_program(externs, fn_def);
    }
    check (lexer.str_eq(kind, "program")) {
        make items: link void = vec.ptr_get(parsed, 2);
        make n: usize = vec.ptr_len(items);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make item: link void = vec.ptr_get(items, i);
            make tag: str = vec.ptr_get(item, 0) as str;
            check (lexer.str_eq(tag, "realm")) {
                make crafts: link void = vec.ptr_get(item, 2);
                make nc: usize = vec.ptr_len(crafts);
                make shift j: usize = 0;
                loopwhile (j < nc) {
                    make fn_def: link void = vec.ptr_get(crafts, j);
                    make ok: bool = semantic.check_fn(fn_def);
                    check (!ok) {
                        writeln("Semantic check failed (realm)");
                        send;
                    }
                    j = j + (1 as usize);
                }
            }
            check (lexer.str_eq(tag, "craft")) {
                make fn_def: link void = vec.ptr_get(item, 1);
                make ok: bool = semantic.check_fn(fn_def);
                check (!ok) {
                    writeln("Semantic check failed (craft)");
                    send;
                }
            }
            i = i + (1 as usize);
        }
        c_code = codegen.emit_program_full(externs, items);
    }
    otherwise {
        make stmts: link void = vec.ptr_get(parsed, 2);
        make result_expr: link void = vec.ptr_get(parsed, 3);
        make names: link void = vec.ptr_new();
        make types: link void = vec.ptr_new();
        make n: usize = vec.ptr_len(stmts);
        make shift i: usize = 0;
        loopwhile (i < n) {
            make pair: link void = vec.ptr_get(stmts, i);
            make vname: str = vec.ptr_get(pair, 0) as str;
            make init_expr: link void = vec.ptr_get(pair, 1);
            make it: str = semantic.check_expr(init_expr, names, types);
            check (strlen(it) == 0) {
                writeln("Semantic check failed (init)");
                send;
            }
            semantic.symtab_put(names, types, vname, it);
            i = i + (1 as usize);
        }
        make ty: str = semantic.check_expr(result_expr, names, types);
        check (strlen(ty) == 0) {
            writeln("Semantic check failed");
            send;
        }
        c_code = codegen.emit_program(externs, stmts, result_expr);
    }
    os.run("mkdir build 2>nul");
    make out_path: str = "build/output.c";
    make ok: i32 = fs.write_all(out_path, c_code);
    check (ok != 1) {
        writeln("Cannot write output.c");
        send;
    }
    writeln("Compiled to build/output.c");
    send;
}