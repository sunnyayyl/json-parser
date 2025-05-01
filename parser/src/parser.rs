use crate::cursor::TokenCursor;
use crate::serialize;
use crate::serialize::Deserialize;
use lexer::{Lexer, LexerToken, LiteralType};
use std::collections::HashMap;
/*pub struct Number {
    integer: i32,
    fraction: i32,
    exponent: i32,
}*/
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Object(Object),
    String(String),
    Number(f64),
    Bool(bool),
    Array(Array),
    Null,
}
impl Into<serialize::RustValue> for Value {
    fn into(self) -> serialize::RustValue {
        match self {
            Value::Object(obj) => obj.deserialize(),
            Value::String(string) => serialize::RustValue::String(string),
            Value::Number(num) => serialize::RustValue::Number(num),
            Value::Array(array) => array.deserialize(),
            Value::Bool(bool) => serialize::RustValue::Bool(bool),
            Value::Null => serialize::RustValue::None,
        }
    }
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
impl Deserialize for Value {
    fn deserialize(self) -> serialize::RustValue {
        self.into()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object(pub(crate) Option<Members>);
impl Deserialize for Object {
    fn deserialize(self) -> serialize::RustValue {
        if let Some(members) = self.0 {
            members.deserialize()
        } else {
            serialize::RustValue::None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Members(Vec<Member>);
impl Deserialize for Members {
    fn deserialize(self) -> serialize::RustValue {
        let mut hashmap = HashMap::with_capacity(self.0.len());
        for member in self.0 {
            hashmap.insert(member.key, member.value.deserialize());
        }
        serialize::RustValue::HashMap(hashmap)
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
pub struct Elements(Vec<Element>);
impl Deserialize for Elements {
    fn deserialize(self) -> serialize::RustValue {
        let mut vec = Vec::with_capacity(self.0.len());
        for i in self.0 {
            vec.push(i.deserialize());
        }
        serialize::RustValue::Vec(vec)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element(Value);
impl Deserialize for Element {
    fn deserialize(self) -> serialize::RustValue {
        self.0.deserialize()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Array(Option<Elements>);
impl Deserialize for Array {
    fn deserialize(self) -> serialize::RustValue {
        if let Some(elements) = self.0 {
            elements.deserialize()
        } else {
            serialize::RustValue::None
        }
    }
}
pub struct Parser<'a> {
    cursor: TokenCursor<'a>,
}
impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            cursor: TokenCursor::new(lexer),
        }
    }
    pub fn parse(mut self) -> Value {
        self.parse_value()
    }
    #[inline]
    fn parse_element(&mut self) -> Element {
        Element(self.parse_value())
    }
    fn parse_elements(&mut self) -> Elements {
        let mut elements = vec![];
        elements.push(self.parse_element());
        while self.cursor.peek() == Some(LexerToken::Comma) {
            self.cursor.next_token();
            elements.push(self.parse_element());
        }
        Elements(elements)
    }
    fn parse_array(&mut self) -> Array {
        self.expect_next(LexerToken::LeftBracket);
        let elements = self.parse_elements();
        self.expect_next(LexerToken::RightBracket);
        Array(Some(elements))
    }
    // parse a number but do not parse an exponent
    fn parse_number(&mut self) -> Option<isize> {
        let token = self.cursor.next_token();
        match token {
            Some(LexerToken::Literal(literal)) => match literal {
                LiteralType::Integer(num) => Some(num),
                literal => panic!("Expected number, got {:?}", literal),
            },
            Some(LexerToken::NegativeSign) => Some(-self.parse_number().unwrap()),
            Some(token) => panic!("Expected number, got {:?}", token),
            None => None,
        }
    }
    fn parse_value(&mut self) -> Value {
        let next = self.cursor.peek();
        match next {
            Some(LexerToken::Literal(literal)) => {
                self.cursor.next_token();
                if self.cursor.peek() == Some(LexerToken::Exponent) {
                    if let LiteralType::Integer(number) = literal {
                        self.cursor.next_token();
                        let exponent = self.parse_number().unwrap();
                        let exp: i32 = exponent.try_into().unwrap();
                        Value::Number((number as f64).powi(exp))
                    } else {
                        panic!("Exponent can only be used on number");
                    }
                } else {
                    literal.into()
                }
            }
            Some(LexerToken::NegativeSign) => Value::Number(self.parse_number().unwrap() as f64),
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
        if let Some(LexerToken::Literal(LiteralType::String(literal))) = self.cursor.peek() {
            self.cursor.next_token();
            key = literal;
        } else {
            panic!(
                "Expected string literal as key, got {:?}",
                self.cursor.next_token()
            )
        }
        self.expect_next(LexerToken::Colon);
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
    use crate::serialize::RustValue;
    #[test]
    fn test_ast() {
        let mut parser = Parser::new(Lexer::new(
            "{\"key\":\"value\", \"key2\\\"\":[1,\"test\", {\"testing\": true}, 10e-1]}", 
            // {"key":"value", "key2\"":[1,"test", {"testing": true}, 10e-1]}
        ));
        let obj = parser.parse();
        assert_eq!(
            obj,
            Value::Object(Object(Some(Members(vec![
                Member::new("key".to_string(), Value::String("value".to_string())),
                Member::new(
                    "key2\"".to_string(),
                    Value::Array(Array(Some(Elements(vec![
                        Element(Value::Number(1.0)),
                        Element(Value::String("test".to_string())),
                        Element(Value::Object(Object(Some(Members(vec![Member::new(
                            "testing".to_string(),
                            Value::Bool(true)
                        )]))))),
                        Element(Value::Number(10.0_f64.powf(-1.0_f64)))
                    ]))))
                )
            ]))))
        );
        assert_eq!(
            obj.deserialize(),
            RustValue::HashMap(HashMap::from([
                ("key".to_string(), RustValue::String("value".to_string())),
                (
                    "key2\"".to_string(),
                    RustValue::Vec(vec![
                        RustValue::Number(1.0),
                        RustValue::String("test".to_string(),),
                        RustValue::HashMap(HashMap::from([(
                            "testing".to_string(),
                            RustValue::Bool(true)
                        )])),
                        RustValue::Number(10.0_f64.powf(-1.0_f64)),
                    ])
                )
            ]))
        );
    }
}
