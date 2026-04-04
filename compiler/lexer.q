// Q++ lexer - hand-written tokenizer for bootstrap compiler
// Output: vec of tokens (link void = token pointer)
// Use: bring "compiler.lexer"; toks = lexer.tokenize(source);

bring "vec";
bring "compiler.tokens";

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
