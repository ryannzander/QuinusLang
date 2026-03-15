//! Lexer (tokenizer) for QuinusLang

use crate::error::{Error, Result};
use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(skip r"//[^\n]*")]
pub enum Token {
    #[token("check")]
    Check,
    #[token("otherwise")]
    Otherwise,
    #[token("for")]
    For,
    #[token("loopwhile")]
    Loopwhile,
    #[token("craft")]
    Craft,
    #[token("send")]
    Send,
    #[token("make")]
    Make,
    #[token("shift")]
    Shift,
    #[token("form")]
    Form,
    #[token("class")]
    Class,
    #[token("extends")]
    Extends,
    #[token("init")]
    Init,
    #[token("new")]
    New,
    #[token("this")]
    This,
    #[token("super")]
    Super,
    #[token("impl")]
    Impl,
    #[token("implements")]
    Implements,
    #[token("realm")]
    Realm,
    #[token("import")]
    Import,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("pub")]
    Pub,
    #[token("priv")]
    Priv,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("==")]
    EqEq,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Bang,

    #[token("=")]
    Eq,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Int(i64),

    #[regex(r"-?[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse().ok())]
    #[regex(r"-?[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse().ok())]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| parse_string_literal(lex.slice()))]
    Str(String),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Bool(bool),
}

fn parse_string_literal(s: &str) -> String {
    let s = &s[1..s.len() - 1];
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(c) => result.push(c),
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Ident(s) => write!(f, "{}", s),
            Token::Int(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Str(s) => write!(f, "\"{}\"", s),
            Token::Bool(b) => write!(f, "{}", b),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<(Token, usize, usize)>,
    pos: usize,
}

impl TokenStream {
    pub fn new(source: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut lexer = Token::lexer(source);

        while let Some(token) = lexer.next() {
            match token {
                Ok(t) => {
                    let start = lexer.span().start;
                    let (l, c) = line_col(source, start);
                    tokens.push((t, l, c));
                }
                Err(()) => {
                    let span = lexer.span();
                    let (l, c) = line_col(source, span.start);
                    return Err(Error::Lexer {
                        line: l,
                        col: c,
                        message: format!("Unexpected character: {:?}", &source[span]),
                    });
                }
            }
        }

        Ok(Self { tokens, pos: 0 })
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|(t, _, _)| t)
    }

    pub fn peek_pos(&self) -> Option<(usize, usize)> {
        self.tokens.get(self.pos).map(|(_, l, c)| (*l, *c))
    }

    pub fn consume(&mut self) -> Option<(Token, usize, usize)> {
        let result = self.tokens.get(self.pos).cloned();
        if result.is_some() {
            self.pos += 1;
        }
        result
    }

    pub fn expect(&mut self, expected: &str) -> Result<(Token, usize, usize)> {
        self.consume().ok_or_else(|| {
            let (line, col) = self.peek_pos().unwrap_or((1, 1));
            Error::Parse {
                line,
                col,
                message: format!("Expected {}", expected),
            }
        })
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

fn line_col(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in source.char_indices() {
        if i >= index {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}


/// Tokenize source code into a stream of tokens.
pub fn tokenize(source: &str) -> Result<TokenStream> {
    TokenStream::new(source)
}
