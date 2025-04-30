use std::str::Chars;

// Borrowed part form rustc_lexer
#[derive(Clone, Debug)]
pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
}
impl<'a> Cursor<'a> {
    pub(crate) fn new(slice: &str) -> Cursor {
        Cursor {
            chars: slice.chars(),
        }
    }
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    pub(crate) fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }
    pub(crate) fn eat_while(&mut self, f: impl Fn(Option<char>) -> bool) {
        while !self.is_eof() && f(self.peek()) {
            self.next_char();
        }
    }
    pub(crate) fn collect_while(&mut self, buff: &mut String, f: impl Fn(Option<char>) -> bool) {
        while !self.is_eof() && f(self.peek()) {
            buff.push(self.next_char().unwrap());
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn everything() {
        let mut cursor = Cursor::new("testing");
        assert_eq!(cursor.is_eof(), false);
        assert_eq!(cursor.peek(), Some('t'));
        assert_eq!(cursor.next_char(), Some('t'));
        assert_eq!(cursor.peek(), Some('e'));
        assert_eq!(cursor.next_char(), Some('e'));
        cursor.eat_while(|c| c.unwrap() != 'g');
        assert_eq!(cursor.next_char(), Some('g'));
        assert_eq!(cursor.peek(), None);
        assert_eq!(cursor.is_eof(), true);
    }
}
