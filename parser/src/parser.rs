use std::clone;

use crate::cursor::TokenCursor;
use lexer::{Lexer, LexerToken, LiteralType};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Object(Object),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}
impl From<LiteralType> for Value {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Float(f) => Value::Number(f),
            LiteralType::Integer(i) => Value::Number(i as f64),
            LiteralType::String(s) => Value::String(s),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    key: String,
    value: Value,
}
impl Member {
    fn new(key: String, value: Value) -> Self {
        return Self { key, value };
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Members(Vec<Member>);

#[derive(Debug, PartialEq, Clone)]
pub struct Object(Option<Members>);
pub struct Parser<'a> {
    cursor: TokenCursor<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            cursor: TokenCursor::new(lexer),
        }
    }
    pub fn parse(&mut self) -> Object {
        self.parse_object().expect("Empty JSON?")
    }
    fn parse_member(cursor: &mut TokenCursor) -> Member {
        let key;
        let value;
        if let Some(LexerToken::Literal(LiteralType::String(literal))) = cursor.next_token() {
            key = literal;
        } else {
            panic!("Expected string literal as key")
        }
        assert!(matches!(cursor.next_token(), Some(LexerToken::Colon)));
        if let Some(LexerToken::Literal(literal)) = cursor.next_token() {
            value = literal;
        } else {
            panic!("Expected literal as value")
        }
        return Member::new(key, value.into());
    }
    fn parse_members(cursor: &mut TokenCursor) -> Members {
        let mut members = vec![];
        members.push(Parser::parse_member(cursor));
        while matches!(cursor.peek(), Some(LexerToken::Comma)) {
            cursor.next_token();
            // pretend
            members.push(Parser::parse_member(cursor));
        }
        return Members(members);
    }
    fn expect(&mut self, token: LexerToken) {
        assert_eq!(self.cursor.next_token(), Some(token));
    }
    pub fn parse_object(&mut self) -> Option<Object> {
        let obj;
        assert_eq!(self.cursor.next_token(), Some(LexerToken::LeftBrace));
        if matches!(self.cursor.peek(), Some(LexerToken::RightBrace)) {
            obj = None;
        } else {
            obj = Some(Object(Some(Parser::parse_members(&mut self.cursor))));
        }
        assert_eq!(self.cursor.next_token(), Some(LexerToken::RightBrace));
        return obj;
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_ast() {
        let mut parser = Parser::new(Lexer::new("{\"key\":\"value\", \"key2\":2}"));
        let obj = parser.parse_object();
        assert_eq!(
            obj,
            Some(Object(Some(Members(vec![
                Member::new("key".to_string(), Value::String("value".to_string())),
                Member::new("key2".to_string(), Value::Number(2.0))
            ]))))
        );
    }
}
