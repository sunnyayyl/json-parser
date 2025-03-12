use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralType {
    String(String),
    Integer(isize),
    Float(f64),
    Null,
}
impl Display for LiteralType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralType::String(s) => write!(f, "String: {}", s),
            LiteralType::Integer(i) => write!(f, "Integer: {}", i),
            LiteralType::Float(v) => write!(f, "Float: {}", v),
            LiteralType::Null => write!(f, "Null"),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum LexerToken {
    Eof,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
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
            LexerToken::LeftBracket => write!(f, "["),
            LexerToken::RightBracket => write!(f, "]"),
            LexerToken::Colon => write!(f, ":"),
            LexerToken::Comma => write!(f, ","),
            LexerToken::Literal(v) => write!(f, "{}", v),
            LexerToken::Illegal => write!(f, "(illegal character)"),
        }
    }
}
