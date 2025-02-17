pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}
impl<'a> Lexer<'a> {
    pub fn new(slice: &'a str) -> Lexer {
        Self {
            cursor: Cursor::new(slice),
        }
    }
    pub fn next_token(&mut self) -> LexerToken {
        self.eat_whitespace();
        let next_char = self.cursor.next_char();
        match next_char {
            Some('{') => LexerToken::LeftBrace,
            Some('}') => LexerToken::RightBrace,
            Some(':') => LexerToken::Colon,
            Some(',')=>LexerToken::Comma,
            Some('"') => {
                let mut collected = String::new();
                loop{
                    if let Some(c) = self.cursor.next_char() {
                        match c {
                            '"' => break,
                            '\\' => {
                                if let Some(escaped_char) = self.cursor.next_char() {
                                    collected.push(escaped_char);
                                }else{
                                    panic!("Unterminated string literal");
                                }
                            }
                            _ => collected.push(c),
                        }
                    }else{
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
            Some(c) => c.is_whitespace() || c=='\n'||c=='\r',
            None => false,
        }
    }
    fn is_numeric(c: Option<char>) -> bool {
        match c {
            Some(c) => c.is_digit(10),
            None => false,
        }
    }
}
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
