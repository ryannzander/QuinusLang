use quinuslang::preprocess;
use quinuslang::{analyze, codegen, parse, parse_with_imports};
use std::path::Path;

#[test]
fn test_full_pipeline_hello() {
    let source = r#"
craft main() -> void {
    make shift x: int = 42;
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 1);
}

#[test]
fn test_full_pipeline_functions() {
    let source = r#"
craft add(a: int, b: int) -> int {
    send a + b;
}
craft main() -> void {
    make shift x: int = add(1, 2);
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 2);
}

#[test]
fn test_full_pipeline_arrays() {
    let source = r#"
craft main() -> void {
    make shift arr: [i32; 3] = { 1, 2, 3 };
    make shift n: usize = len(arr);
    print(n);
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 1);
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("int32_t arr[3]"));
    assert!(c_code.contains("len") || c_code.contains("3") || c_code.contains("sizeof"));
}

#[test]
fn test_builtins_write_writeln() {
    let source = r#"
craft main() -> void {
    write(1);
    writeln(2);
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("printf"));
}

#[test]
fn test_parse_with_imports_no_import() {
    let source = r#"craft main() -> void { send; }"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    assert_eq!(program.items.len(), 1);
}

#[test]
fn test_parse_error() {
    let bad = r#"craft main() { "#;
    let result = parse(bad);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Parse") || format!("{}", err).contains("parse"));
}

#[test]
fn test_semantic_error_undefined_var() {
    let source = r#"
craft main() -> void {
    make x: i32 = y;
    send;
}
"#;
    let program = parse(source).unwrap();
    let result = analyze(&program);
    assert!(result.is_err());
}

#[test]
fn test_semantic_error_type_mismatch() {
    let source = r#"
craft main() -> void {
    make x: i32 = "hello";
    send;
}
"#;
    let program = parse(source).unwrap();
    let result = analyze(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("Cannot assign") || format!("{}", err).contains("assign"));
}

#[test]
fn test_semantic_error_wrong_arg_count() {
    let source = r#"
craft main() -> void {
    make n: usize = len(1, 2);
    send;
}
"#;
    let program = parse(source).unwrap();
    let result = analyze(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("len") || format!("{}", err).contains("argument"));
}

#[test]
fn test_semantic_error_const_type_mismatch() {
    let source = r#"
eternal PI: i32 = "not a number";
craft main() -> void { send; }
"#;
    let program = parse(source).unwrap();
    let result = analyze(&program);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{}", err).contains("constant") || format!("{}", err).contains("assign"));
}

#[test]
fn test_formatter_roundtrip() {
    use quinuslang::fmt;
    let source = r#"craft main() -> void {
    check (x > 0) {
        print(1);
    }
    send;
}
"#;
    let program = parse(source).unwrap();
    let formatted = fmt::format_program(&program);
    let reparsed = parse(&formatted).unwrap();
    assert_eq!(program.items.len(), reparsed.items.len());
}

#[test]
fn test_formatter_roundtrip_defer() {
    use quinuslang::fmt;
    let source = r#"craft main() -> void {
    defer { print(1); }
    send;
}
"#;
    let program = parse(source).unwrap();
    let formatted = fmt::format_program(&program);
    let reparsed = parse(&formatted).unwrap();
    assert_eq!(program.items.len(), reparsed.items.len());
}

#[test]
fn test_formatter_roundtrip_foreach() {
    use quinuslang::fmt;
    let source = r#"craft main() -> void {
    make shift arr: [i32; 3] = { 1, 2, 3 };
    foreach x in arr { print(x); }
    send;
}
"#;
    let program = parse(source).unwrap();
    let formatted = fmt::format_program(&program);
    let reparsed = parse(&formatted).unwrap();
    assert_eq!(program.items.len(), reparsed.items.len());
}

#[test]
fn test_fs_module() {
    let source = r#"
bring "fs";
craft main() -> void {
    make f: link void = fs.open_file("test.txt", "r");
    fs.close(f);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("fopen") || c_code.contains("fclose"));
}

#[test]
fn test_math_module() {
    let source = r#"
bring "math";
craft main() -> void {
    make a: i32 = math.min_i32(1, 2);
    make b: f64 = math.sqrt_f64(4.0);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("sqrt") || c_code.contains("fmin"));
}

#[test]
fn test_math_checked_arithmetic() {
    let source = r#"
bring "math";
craft main() -> void {
    make r: Result(i32, i32) = math.add_checked_i32(1, 2);
    choose (r) {
        Ok(v) => { print(v); }
        Err(_) => { print(0); }
    }
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("ql_add_checked_i32"));
    assert!(c_code.contains("__builtin_add_overflow") || c_code.contains("INT32_MIN"));
}

#[test]
fn test_str_module() {
    let source = r#"
bring "str";
craft main() -> void {
    make t: str = str.trim("  hi  ");
    make c: str = str.concat("a", "b");
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("ql_str_trim") || c_code.contains("str_trim"));
    assert!(c_code.contains("ql_str_concat") || c_code.contains("str_concat"));
}

#[test]
fn test_time_module() {
    let source = r#"
bring "time";
craft main() -> void {
    make t: i64 = time.now();
    print(t);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("ql_time_now") || c_code.contains("time(0)"));
}

#[test]
fn test_rand_module() {
    let source = r#"
bring "rand";
craft main() -> void {
    rand.seed(123);
    make r: i32 = rand.next();
    print(r);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("rand") && c_code.contains("srand"));
}

#[test]
fn test_simd_module() {
    let source = r#"
bring "simd";
craft main() -> void {
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("xmmintrin"));
}

#[test]
fn test_arena_module() {
    let source = r#"
bring "arena";
craft main() -> void {
    make p: link void = arena.alloc(64);
    arena.dealloc(p);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("malloc") || c_code.contains("arena"));
}

#[test]
fn test_os_cwd() {
    let source = r#"
bring "os";
craft main() -> void {
    make dir: str = os.cwd();
    print(dir);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("getcwd") || c_code.contains("_getcwd"));
}

#[test]
fn test_string_interpolation() {
    let source = r#"
craft main() -> void {
    make name: str = "world";
    print(`Hello, ${name}!`);
    make x: i32 = 42;
    print(`x = ${x}`);
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("Hello, %s!"));
    assert!(c_code.contains("x = %ld"));
}

#[test]
fn test_result_type() {
    let source = r#"
craft maybe_parse(s: str) -> Result(i32, i32) {
    send Ok(42);
}
craft main() -> void {
    make r: Result(i32, i32) = maybe_parse("x");
    choose (r) {
        Ok(v) => { print(v); }
        Err(e) => { print(e); }
    }
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("ql_result"));
    assert!(c_code.contains("is_ok"));
    assert!(c_code.contains(".u.val"));
    assert!(c_code.contains(".u.err"));
}

#[test]
fn test_bitfields() {
    let source = r#"
form Flags {
    a: u32 : 8,
    b: u32 : 8,
    c: u32 : 16,
}
craft main() -> void {
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains(": 8"));
    assert!(c_code.contains(": 16"));
}

#[test]
fn test_move_semantics() {
    let source = r#"
craft main() -> void {
    make x: i32 = 42;
    make y: i32 = move x;
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("42"));
}

#[test]
fn test_inline_cblock() {
    let source = r#"
craft main() -> void {
    hazard {
        cblock { " int _x = 42; " };
        cblock { " _x = 0; " };
    }
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("int _x = 42"));
    assert!(c_code.contains("_x = 0"));
}

#[test]
fn test_compile_flags() {
    let source = r#"
#define FOO
#if FOO
craft main() -> void {
    send;
}
#else
craft main() -> void {
    make x: i32 = 1;
    send;
}
#endif
"#;
    let flattened = preprocess::preprocess_with_defines(source, Path::new("."), &[]).unwrap();
    let program = parse(&flattened).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 1);
    // With FOO defined, we get the first main() (empty body). No make x.
    assert!(!flattened.contains("make x"));
}

#[test]
fn test_compile_flags_undefined() {
    let source = r#"
#if FOO
craft main() -> void { send; }
#else
craft main() -> void {
    make x: i32 = 42;
    send;
}
#endif
"#;
    let flattened = preprocess::preprocess_with_defines(source, Path::new("."), &[]).unwrap();
    let program = parse(&flattened).unwrap();
    let _annotated = analyze(&program).unwrap();
    assert!(flattened.contains("make x"));
}

#[test]
fn test_compile_flags_define_arg() {
    let source = r#"
#if DEBUG
craft main() -> void {
    make x: i32 = 1;
    send;
}
#else
craft main() -> void { send; }
#endif
"#;
    let defines = vec!["DEBUG".to_string()];
    let flattened = preprocess::preprocess_with_defines(source, Path::new("."), &defines).unwrap();
    assert!(flattened.contains("make x"));
}

#[test]
fn test_digit_separators() {
    let source = r#"
craft main() -> void {
    make x: i32 = 1_000_000;
    print(x);
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("1000000"));
}

#[test]
fn test_assert_with_message() {
    let source = r#"
craft main() -> void {
    assert(true);
    assert(1 == 1, "should not fail");
    send;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("assertion failed"));
}

#[test]
fn test_path_module() {
    let source = r#"
bring "path";
craft main() -> void {
    make p: str = path.join("a", "b");
    print(p);
    send;
}
"#;
    let program = parse_with_imports(source, Path::new("."), &[]).unwrap();
    let annotated = analyze(&program).unwrap();
    let c_code = codegen::c::generate(&annotated).unwrap();
    assert!(c_code.contains("str_concat") || c_code.contains("ql_str_concat"));
}
