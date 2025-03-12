
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
            LiteralType::Null => Value::Null,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Member {
    key: String,
    value: Value,
}
impl Member {
    pub fn new(key: String, value: Value) -> Self {
        Self { key, value }
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
    fn parse_member(&mut self) -> Member {
        let key;
        let value;
        if let Some(LexerToken::Literal(LiteralType::String(literal))) = self.cursor.next_token() {
            key = literal;
        } else {
            panic!("Expected string literal as key")
        }
        self.expect_next(LexerToken::Colon);
        if let Some(LexerToken::Literal(literal)) = self.cursor.next_token() {
            value = literal;
        } else {
            panic!("Expected literal as value")
        }
        Member::new(key, value.into())
    }
    fn parse_members(&mut self) -> Members {
        let mut members = vec![];
        members.push(self.parse_member());
        while matches!(self.cursor.peek(), Some(LexerToken::Comma)) {
            self.cursor.next_token();
            members.push(self.parse_member());
        }
        Members(members)
    }
    fn expect_next(&mut self, token: LexerToken) {
        let got = self
            .cursor
            .next_token()
            .expect(&format!("Expected Some({}) token, got None", token));
        assert_eq!(got, token, "Expected {}, got {}", token, got);
    }
    fn parse_object(&mut self) -> Option<Object> {
        let obj;
        self.expect_next(LexerToken::LeftBrace);
        if matches!(self.cursor.peek(), Some(LexerToken::RightBrace)) {
            obj = None;
        } else {
            obj = Some(Object(Some(self.parse_members())));
        }
        self.expect_next(LexerToken::RightBrace);
        obj
    }
}
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_ast() {
        let mut parser = Parser::new(Lexer::new("{\"key\":\"value\", \"key2\\\"\":2}"));
        let obj = parser.parse_object();
        assert_eq!(
            obj,
            Some(Object(Some(Members(vec![
                Member::new("key".to_string(), Value::String("value".to_string())),
                Member::new("key2\"".to_string(), Value::Number(2.0))
            ]))))
        );
    }
}
