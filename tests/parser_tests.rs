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

#[test]
fn test_parse_array_type() {
    let source = r#"
craft main() -> void {
    make shift arr: [i32; 5] = { 1, 2, 3, 4, 5 };
    send;
}
"#;
    let program = parse(source).unwrap();
    assert_eq!(program.items.len(), 1);
    use quinuslang::ast::{TopLevelItem, Stmt, Type};
    if let TopLevelItem::Fn(f) = &program.items[0] {
        if let Stmt::VarDecl { ty: Some(t), .. } = &f.body[0] {
            assert!(matches!(t, Type::ArraySized(_, 5)));
        }
    }
}

#[test]
fn test_parse_array_init() {
    let source = r#"make shift x: [i32; 3] = { 10, 20, 30 };"#;
    let program = parse(&format!("craft main() -> void {{ {} send; }}", source)).unwrap();
    assert_eq!(program.items.len(), 1);
}
