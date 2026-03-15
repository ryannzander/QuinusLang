use quinuslang::{analyze, parse};

#[test]
fn test_full_pipeline_hello() {
    let source = r#"
func main() -> void {
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
func add(a: int, b: int) -> int {
    return a + b;
}
func main() -> void {
    var x: int = add(1, 2);
    return;
}
"#;
    let program = parse(source).unwrap();
    let annotated = analyze(&program).unwrap();
    assert_eq!(annotated.program.items.len(), 2);
}
