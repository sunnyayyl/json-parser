use lexer::{Lexer, LexerToken};
use crate::cursor::TokenCursor;

enum Value{
    Object(Object),
    String(String),
    Number(f64),
    Bool(bool),
    Null
}
struct Member{
    key : String,
    value: Value,
}
struct Members{
    members: Vec<Member>,
}

struct Object(Members);

struct Parser<'a> {
    cursor: TokenCursor<'a>
}
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self{
            cursor: TokenCursor::new(lexer)
        }
    }
    pub fn parse(&mut self) {


    }
    pub fn parse_object(&mut self) -> Object{
        assert_eq!(self.cursor.next_token(), Some(LexerToken::LeftBrace));
        let obj = Object;
        assert_eq!(self.cursor.next_token(), Some(LexerToken::RightBrace));
        todo!()
    }
}