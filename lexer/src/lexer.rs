use crate::cursor::Cursor;
use crate::token::{LexerToken, LiteralType};
use std::ops::Add;
fn collect_while(cursor: &mut Cursor, f: impl Fn(char, usize) -> bool) -> String {
    let mut pos = 0;
    let mut collected = String::new();
    'collect_string: while let Some(c) = cursor.peek() {
        if f(c, pos) {
            break 'collect_string;
        }
        collected.push(c);
        cursor.next_char();
        pos += 1;
    }
    collected
}
pub struct IntoIter<'a>(Lexer<'a>);
impl Iterator for IntoIter<'_> {
    type Item = LexerToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next_token() {
            LexerToken::Eof => None,
            token => Some(token),
        }
    }
}
#[derive(Debug,Clone)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}
impl<'a> Lexer<'a> {
    pub fn new(slice: &'a str) -> Lexer<'a> {
        Self {
            cursor: Cursor::new(slice),
        }
    }
    pub fn next_token(&mut self) -> LexerToken {
        self.eat_whitespace();
        if self.cursor.is_eof() {
            return LexerToken::Eof;
        }
        let next_char = self.cursor.next_char();
        match next_char {
            Some('{') => LexerToken::LeftBrace,
            Some('}') => LexerToken::RightBrace,
            Some(':') => LexerToken::Colon,
            Some(',') => LexerToken::Comma,
            Some('"') => {
                let mut collected = String::new();
                loop {
                    if let Some(c) = self.cursor.next_char() {
                        match c {
                            '"' => break,
                            '\\' => {
                                if let Some(escaped_char) = self.cursor.next_char() {
                                    collected.push(escaped_char);
                                } else {
                                    panic!("Unterminated string literal");
                                }
                            }
                            _ => collected.push(c),
                        }
                    } else {
                        panic!("Unterminated string literal");
                    }
                }
                LexerToken::Literal(LiteralType::String(collected))
            }
            Some(c) if c.is_digit(10) => LexerToken::Literal(LiteralType::Integer(
                c.to_string()
                    .add(&*collect_while(&mut self.cursor, |c, _| !c.is_digit(10)))
                    .parse()
                    .unwrap(),
            )),
            _ => LexerToken::Illegal,
        }
    }

    fn eat_whitespace(&mut self) {
        self.cursor.eat_while(Self::is_whitespace);
    }
    fn is_whitespace(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_whitespace() || c == '\n' || c == '\r',
            None => false,
        }
    }
}
impl<'a> IntoIterator for Lexer<'a> {
    type Item = LexerToken;
    type IntoIter = IntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token(){
        let mut lexer = Lexer::new("{\
        \"testing\": \"a \\\"string\\\"\",
        \"number\\\\\": 1234
        }");
        assert_eq!(lexer.next_token(), LexerToken::LeftBrace);
        assert_eq!(lexer.next_token(), LexerToken::Literal(LiteralType::String(String::from("testing"))));
        assert_eq!(lexer.next_token(), LexerToken::Colon);
        assert_eq!(lexer.next_token(), LexerToken::Literal(LiteralType::String(String::from("a \"string\""))));
        assert_eq!(lexer.next_token(), LexerToken::Comma);
        assert_eq!(lexer.next_token(), LexerToken::Literal(LiteralType::String(String::from("number\\"))));
        assert_eq!(lexer.next_token(), LexerToken::Colon);
        assert_eq!(lexer.next_token(), LexerToken::Literal(LiteralType::Integer(1234)));
        assert_eq!(lexer.next_token(), LexerToken::RightBrace);
        assert_eq!(lexer.next_token(), LexerToken::Eof);
    }
}