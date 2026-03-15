// QuinusLang bootstrap compiler driver
// Pipeline: read source -> lex -> parse -> semantic -> codegen -> write C -> invoke cc
// Usage: build this, then run with input file (default: input.q)
// Build: cargo run -- build compiler/main.q

bring "fs";
bring "vec";
bring "compiler.lexer";
bring "compiler.parser";
bring "compiler.semantic";
bring "compiler.codegen";
bring "os";

extern craft strlen(s: str) -> usize;

craft main() -> void {
    make path: str = "input.q";
    make f: link void = fs.open_file(path, "r");
    check (f == 0) {
        writeln("Cannot open input.q");
        send;
    }
    make src: str = fs.read_all(f);
    fs.close(f);
    make parsed: link void = parser.parse(src);
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
