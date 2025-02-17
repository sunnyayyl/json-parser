use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub (crate) enum LiteralType {
    String(String),
    Integer(isize),
    Float(f64),
}
impl Display for LiteralType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "String: {}", s),
            LiteralType::Integer(i) => write!(f, "Integer: {}", i),
            LiteralType::Float(v) => write!(f, "Float: {}", v),
        }
    }
}
#[derive(Debug)]
pub(crate) enum LexerToken {
    Eof,
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    Literal(LiteralType),
    Illegal,
}
impl Display for LexerToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Token) ")?;
        match self {
            LexerToken::Eof => write!(f, "EOF"),
            LexerToken::LeftBrace => write!(f, "{{"),
            LexerToken::RightBrace => write!(f, "}}"),
            LexerToken::Colon => write!(f, ":"),
            LexerToken::Comma => write!(f, ","),
            LexerToken::Literal(v) => write!(f, "{}", v),
            LexerToken::Illegal => write!(f, "(illegal character)"),
        }
    }
}
