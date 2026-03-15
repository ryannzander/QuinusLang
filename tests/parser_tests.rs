use quinuslang::parse;

#[test]
fn test_parse_simple_fn() {
    let source = r#"
func main() -> void {
    var x: int = 42;
    return;
}
"#;
    let program = parse(source).unwrap();
    assert_eq!(program.items.len(), 1);
}
