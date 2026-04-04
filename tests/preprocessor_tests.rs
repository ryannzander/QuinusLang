use qpp::preprocess;
use qpp::{analyze, parse};
use std::path::Path;

fn preprocess_ok(src: &str, defines: &[&str]) -> String {
    let defs: Vec<String> = defines.iter().map(|s| s.to_string()).collect();
    preprocess::preprocess_with_defines(src, Path::new("."), &defs)
        .unwrap_or_else(|e| panic!("Preprocess failed: {}", e))
}

#[test]
fn pp_no_directives() {
    let src = "craft main() -> void { send; }";
    let result = preprocess_ok(src, &[]);
    assert!(result.contains("craft main"));
}

#[test]
fn pp_define_and_ifdef() {
    let src = r#"
#define FOO
#if FOO
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { send; }
#endif
"#;
    let result = preprocess_ok(src, &[]);
    assert!(result.contains("make x"));
}

#[test]
fn pp_undefined_ifdef_uses_else() {
    let src = r#"
#if BAR
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { make y: i32 = 2; send; }
#endif
"#;
    let result = preprocess_ok(src, &[]);
    assert!(!result.contains("make x"));
    assert!(result.contains("make y"));
}

#[test]
fn pp_define_via_arg() {
    let src = r#"
#if DEBUG
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { send; }
#endif
"#;
    let result = preprocess_ok(src, &["DEBUG"]);
    assert!(result.contains("make x"));
}

#[test]
fn pp_define_arg_not_set() {
    let src = r#"
#if DEBUG
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { make y: i32 = 2; send; }
#endif
"#;
    let result = preprocess_ok(src, &[]);
    assert!(result.contains("make y"));
    assert!(!result.contains("make x"));
}

#[test]
fn pp_nested_directives() {
    let src = r#"
#define A
#define B
#if A
#if B
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { make y: i32 = 2; send; }
#endif
#else
craft main() -> void { send; }
#endif
"#;
    let result = preprocess_ok(src, &[]);
    assert!(result.contains("make x"));
}

#[test]
fn pp_output_compiles() {
    let src = r#"
#define FEATURE
#if FEATURE
craft main() -> void {
    make x: i32 = 42;
    print(x);
    send;
}
#else
craft main() -> void { send; }
#endif
"#;
    let flattened = preprocess_ok(src, &[]);
    let program = parse(&flattened).unwrap();
    let _annotated = analyze(&program).unwrap();
}

#[test]
fn pp_multiple_defines() {
    let src = r#"
#define A
#define B
#if A
craft foo() -> void { send; }
#endif
#if B
craft bar() -> void { send; }
#endif
craft main() -> void { send; }
"#;
    let result = preprocess_ok(src, &[]);
    assert!(result.contains("craft foo"));
    assert!(result.contains("craft bar"));
}

#[test]
fn pp_cli_define_overrides() {
    let src = r#"
#if RELEASE
craft main() -> void { make x: i32 = 1; send; }
#else
craft main() -> void { make y: i32 = 2; send; }
#endif
"#;
    let with_release = preprocess_ok(src, &["RELEASE"]);
    assert!(with_release.contains("make x"));

    let without_release = preprocess_ok(src, &[]);
    assert!(without_release.contains("make y"));
}
