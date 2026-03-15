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
