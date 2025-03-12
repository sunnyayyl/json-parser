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
#[derive(Debug, Clone)]
pub struct IntoIter<'a> {
    lexer: Lexer<'a>,
    pub eof: bool,
}
impl Iterator for IntoIter<'_> {
    type Item = LexerToken;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next_token() {
            LexerToken::Eof => {
                self.eof = true;
                None
            }
            token => Some(token),
        }
    }
}
#[derive(Debug, Clone)]
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
            Some('[') => LexerToken::LeftBracket,
            Some(']') => LexerToken::RightBracket,
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
            Some(c) if c.is_alphabetic() =>{
                if self.cursor.is_match("ull"){
                    LexerToken::Literal(LiteralType::Null)
                
                }else{
                    panic!("Unexpected charater {}",c)
                }
            }
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
        IntoIter {
            lexer: self,
            eof: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token() {
        let mut lexer = Lexer::new(
            "{\
        \"testing\": \"a \\\"string\\\"\",
        \"numbers\\\\\": [1,\"2\",null]
        }",
        );
        assert_eq!(lexer.next_token(), LexerToken::LeftBrace);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::String(String::from("testing")))
        );
        assert_eq!(lexer.next_token(), LexerToken::Colon);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::String(String::from("a \"string\"")))
        );
        assert_eq!(lexer.next_token(), LexerToken::Comma);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::String(String::from("numbers\\")))
        );
        assert_eq!(lexer.next_token(), LexerToken::Colon);
        assert_eq!(lexer.next_token(), LexerToken::LeftBracket);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::Integer(1))
        );
        assert_eq!(lexer.next_token(), LexerToken::Comma);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::String(String::from("2")))
        );
        assert_eq!(lexer.next_token(), LexerToken::Comma);
        assert_eq!(
            lexer.next_token(),
            LexerToken::Literal(LiteralType::Null)
        );
        assert_eq!(lexer.next_token(), LexerToken::RightBracket);
        assert_eq!(lexer.next_token(), LexerToken::RightBrace);
        assert_eq!(lexer.next_token(), LexerToken::Eof);
    }
}
