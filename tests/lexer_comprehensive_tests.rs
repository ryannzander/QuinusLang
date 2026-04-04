use qpp::lexer::{tokenize, Token};

fn collect_tokens(src: &str) -> Vec<Token> {
    let mut stream = tokenize(src).unwrap();
    let mut tokens = vec![];
    while let Some((t, _, _)) = stream.consume() {
        tokens.push(t);
    }
    tokens
}

#[test]
fn lex_integer_literals() {
    let tokens = collect_tokens("0 1 42 9999");
    assert!(matches!(tokens[0], Token::Int(0)));
    assert!(matches!(tokens[1], Token::Int(1)));
    assert!(matches!(tokens[2], Token::Int(42)));
    assert!(matches!(tokens[3], Token::Int(9999)));
}

#[test]
fn lex_negative_integer() {
    let tokens = collect_tokens("-7");
    assert!(matches!(tokens[0], Token::Int(-7)));
}

#[test]
fn lex_digit_separators() {
    let tokens = collect_tokens("1_000 10_000_000");
    assert!(matches!(tokens[0], Token::Int(1000)));
    assert!(matches!(tokens[1], Token::Int(10000000)));
}

#[test]
fn lex_float_literals() {
    let tokens = collect_tokens("3.14 0.5 100.0");
    assert!(matches!(tokens[0], Token::Float(f) if (f - 3.14).abs() < 1e-9));
    assert!(matches!(tokens[1], Token::Float(f) if (f - 0.5).abs() < 1e-9));
    assert!(matches!(tokens[2], Token::Float(f) if (f - 100.0).abs() < 1e-9));
}

#[test]
fn lex_string_literals() {
    let tokens = collect_tokens(r#""hello" "world" """#);
    assert!(matches!(&tokens[0], Token::Str(s) if s == "hello"));
    assert!(matches!(&tokens[1], Token::Str(s) if s == "world"));
    assert!(matches!(&tokens[2], Token::Str(s) if s.is_empty()));
}

#[test]
fn lex_boolean_literals() {
    let tokens = collect_tokens("true false");
    assert!(matches!(tokens[0], Token::Bool(true)));
    assert!(matches!(tokens[1], Token::Bool(false)));
}

#[test]
fn lex_identifiers() {
    let tokens = collect_tokens("foo bar_baz _x x1");
    assert!(matches!(&tokens[0], Token::Ident(s) if s == "foo"));
    assert!(matches!(&tokens[1], Token::Ident(s) if s == "bar_baz"));
    assert!(matches!(&tokens[2], Token::Ident(s) if s == "_x"));
    assert!(matches!(&tokens[3], Token::Ident(s) if s == "x1"));
}

#[test]
fn lex_keywords() {
    let tokens = collect_tokens("craft send make shift check otherwise for loopwhile foreach in");
    assert!(matches!(tokens[0], Token::Craft));
    assert!(matches!(tokens[1], Token::Send));
    assert!(matches!(tokens[2], Token::Make));
    assert!(matches!(tokens[3], Token::Shift));
    assert!(matches!(tokens[4], Token::Check));
    assert!(matches!(tokens[5], Token::Otherwise));
    assert!(matches!(tokens[6], Token::For));
    assert!(matches!(tokens[7], Token::Loopwhile));
    assert!(matches!(tokens[8], Token::Foreach));
    assert!(matches!(tokens[9], Token::In));
}

#[test]
fn lex_more_keywords() {
    let tokens = collect_tokens("form state class extends init new this super impl extern bring");
    assert!(matches!(tokens[0], Token::Form));
    assert!(matches!(tokens[1], Token::State));
    assert!(matches!(tokens[2], Token::Class));
    assert!(matches!(tokens[3], Token::Extends));
    assert!(matches!(tokens[4], Token::Init));
    assert!(matches!(tokens[5], Token::New));
    assert!(matches!(tokens[6], Token::This));
    assert!(matches!(tokens[7], Token::Super));
    assert!(matches!(tokens[8], Token::Impl));
    assert!(matches!(tokens[9], Token::Extern));
    assert!(matches!(tokens[10], Token::Bring));
}

#[test]
fn lex_control_keywords() {
    let tokens = collect_tokens("stop skip eternal anchor hazard defer choose try catch");
    assert!(matches!(tokens[0], Token::Stop));
    assert!(matches!(tokens[1], Token::Skip));
    assert!(matches!(tokens[2], Token::Eternal));
    assert!(matches!(tokens[3], Token::Anchor));
    assert!(matches!(tokens[4], Token::Hazard));
    assert!(matches!(tokens[5], Token::Defer));
    assert!(matches!(tokens[6], Token::Choose));
    assert!(matches!(tokens[7], Token::Try));
    assert!(matches!(tokens[8], Token::Catch));
}

#[test]
fn lex_operators() {
    let tokens = collect_tokens("+ - * / % == != < <= > >= && ||");
    assert!(matches!(tokens[0], Token::Plus));
    assert!(matches!(tokens[1], Token::Minus));
    assert!(matches!(tokens[2], Token::Star));
    assert!(matches!(tokens[3], Token::Slash));
    assert!(matches!(tokens[4], Token::Percent));
    assert!(matches!(tokens[5], Token::EqEq));
    assert!(matches!(tokens[6], Token::Ne));
    assert!(matches!(tokens[7], Token::Lt));
    assert!(matches!(tokens[8], Token::Le));
    assert!(matches!(tokens[9], Token::Gt));
    assert!(matches!(tokens[10], Token::Ge));
    assert!(matches!(tokens[11], Token::AndAnd));
    assert!(matches!(tokens[12], Token::OrOr));
}

#[test]
fn lex_punctuation() {
    let tokens = collect_tokens("( ) { } [ ] ; : , . ->");
    assert!(matches!(tokens[0], Token::LParen));
    assert!(matches!(tokens[1], Token::RParen));
    assert!(matches!(tokens[2], Token::LBrace));
    assert!(matches!(tokens[3], Token::RBrace));
    assert!(matches!(tokens[4], Token::LBracket));
    assert!(matches!(tokens[5], Token::RBracket));
    assert!(matches!(tokens[6], Token::Semicolon));
    assert!(matches!(tokens[7], Token::Colon));
    assert!(matches!(tokens[8], Token::Comma));
    assert!(matches!(tokens[9], Token::Dot));
    assert!(matches!(tokens[10], Token::Arrow));
}

#[test]
fn lex_assignment_operators() {
    let tokens = collect_tokens("= !");
    assert!(matches!(tokens[0], Token::Eq));
    assert!(matches!(tokens[1], Token::Bang));
}

#[test]
fn lex_comments_skipped() {
    let tokens = collect_tokens("42 // this is a comment\n43");
    assert!(matches!(tokens[0], Token::Int(42)));
    assert!(matches!(tokens[1], Token::Int(43)));
}

#[test]
fn lex_backtick_string() {
    let tokens = collect_tokens("`hello`");
    assert!(matches!(&tokens[0], Token::InterpolateStr(s) if s == "hello"));
}

#[test]
fn lex_empty_input() {
    let tokens = collect_tokens("");
    assert!(tokens.is_empty());
}

#[test]
fn lex_whitespace_only() {
    let tokens = collect_tokens("   \n\t  ");
    assert!(tokens.is_empty());
}

#[test]
fn lex_type_keywords() {
    let tokens = collect_tokens("int float bool str void link");
    assert!(matches!(&tokens[0], Token::Ident(s) if s == "int"));
    assert!(matches!(&tokens[1], Token::Ident(s) if s == "float"));
    assert!(matches!(&tokens[2], Token::Ident(s) if s == "bool"));
    assert!(matches!(&tokens[3], Token::Ident(s) if s == "str"));
    assert!(matches!(&tokens[4], Token::Ident(s) if s == "void"));
    assert!(matches!(tokens[5], Token::Link));
}
