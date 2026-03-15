use quinuslang::{analyze, parse};

#[test]
fn test_full_pipeline_hello() {
    let source = r#"
fn main() -> void {
    var x: int = 42;
    return;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 1);
}

#[test]
fn test_full_pipeline_functions() {
    let source = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}
fn main() -> void {
    var x: int = add(1, 2);
    return;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 2);
}
