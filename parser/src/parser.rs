use crate::cursor::TokenCursor;
use lexer::{Lexer, LexerToken, LiteralType};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Object(Object),
    String(String),
    Number(f64),
    Bool(bool),
    Array(Array),
    Null,
}
impl From<LiteralType> for Value {
    fn from(value: LiteralType) -> Self {
        match value {
            LiteralType::Float(f) => Value::Number(f),
            LiteralType::Integer(i) => Value::Number(i as f64),
            LiteralType::String(s) => Value::String(s),
            LiteralType::Null => Value::Null,
            LiteralType::Bool(b) => Value::Bool(b),
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
#[derive(Debug, PartialEq, Clone)]

pub struct Elements(Vec<Element>);
#[derive(Debug, PartialEq, Clone)]

pub struct Element(Value);
#[derive(Debug, PartialEq, Clone)]

pub struct Array(Option<Elements>);
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
    #[inline]
    fn parse_element(&mut self) -> Element {
        Element(self.parse_value())
    }
    fn parse_elements(&mut self) -> Elements {
        let mut elemets = vec![];
        elemets.push(self.parse_element());
        while self.cursor.peek() == Some(LexerToken::Comma) {
            self.cursor.next_token();
            elemets.push(self.parse_element());
        }
        Elements(elemets)
    }
    fn parse_array(&mut self) -> Array {
        self.expect_next(LexerToken::LeftBracket);
        let elements = self.parse_elements();
        self.expect_next(LexerToken::RightBracket);
        Array(Some(elements))
    }
    fn parse_value(&mut self) -> Value {
        let next = self.cursor.peek();
        match next {
            Some(LexerToken::Literal(literal)) => {
                self.cursor.next_token();
                literal.into()
            }
            Some(LexerToken::LeftBrace) => Value::Object(
                self.parse_object()
                    .expect("Expected to find an object, got None"),
            ),
            Some(LexerToken::LeftBracket) => Value::Array(self.parse_array()),
            Some(_) => panic!("Unexpected token {:?}", next),
            None => panic!("Value expected"),
        }
    }
    fn parse_member(&mut self) -> Member {
        let key;
        if let Some(LexerToken::Literal(LiteralType::String(literal))) = self.cursor.next_token() {
            key = literal;
        } else {
            panic!("Expected string literal as key")
        }
        self.expect_next(LexerToken::Colon);
        /*if let Some(LexerToken::Literal(literal)) = self.cursor.next_token() {
            value = literal;
        } else {
            panic!("Expected literal as value")
        }*/
        let value = self.parse_value();
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
        let mut parser = Parser::new(Lexer::new(
            "{\"key\":\"value\", \"key2\\\"\":[1,\"test\", {\"testing\": true}]}",
        ));
        let obj = parser.parse_object();
        assert_eq!(
            obj,
            Some(Object(Some(Members(vec![
                Member::new("key".to_string(), Value::String("value".to_string())),
                Member::new(
                    "key2\"".to_string(),
                    Value::Array(Array(Some(Elements(vec![
                        Element(Value::Number(1.0)),
                        Element(Value::String("test".to_string())),
                        Element(Value::Object(Object(Some(Members(vec![Member::new(
                            "testing".to_string(),
                            Value::Bool(true)
                        )])))))
                    ]))))
                )
            ]))))
        );
    }
}
