use quinuslang::parse;

#[test]
fn test_parse_simple_fn() {
    let source = r#"
craft main() -> void {
    make shift x: int = 42;
    send;
}
"#;
    let program = parse(source).unwrap();
    assert_eq!(program.items.len(), 1);
}
