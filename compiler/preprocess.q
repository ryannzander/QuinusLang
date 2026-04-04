// Q++ preprocessor - bring flattening
// Resolves bring statements recursively, outputs flattened source
// Port of src/preprocess.rs

bring "fs";
bring "vec";
bring "str";
bring "compiler.lexer";

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
