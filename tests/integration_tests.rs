use quinuslang::{analyze, parse};

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
