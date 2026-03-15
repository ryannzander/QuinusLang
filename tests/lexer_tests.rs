use quinuslang::lexer::tokenize;

#[test]
fn test_tokenize_integers() {
    let mut stream = tokenize("42 -17 0").unwrap();
    assert!(stream.peek().is_some());
    let (t, _, _) = stream.consume().unwrap();
    assert!(matches!(t, quinuslang::lexer::Token::Int(42)));
}

#[test]
fn test_tokenize_identifiers() {
    let mut stream = tokenize("foo bar_baz").unwrap();
    let (t, _, _) = stream.consume().unwrap();
    assert!(matches!(t, quinuslang::lexer::Token::Ident(s) if s == "foo"));
    let (t, _, _) = stream.consume().unwrap();
    assert!(matches!(t, quinuslang::lexer::Token::Ident(s) if s == "bar_baz"));
}
