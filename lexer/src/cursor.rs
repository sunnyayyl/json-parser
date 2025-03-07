use std::str::Chars;

// Borrowed part form rustc_lexer
#[derive(Clone, Debug)]
pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    len_remaining: usize,
}
impl<'a> Cursor<'a> {
    pub(crate) fn new(slice: &str) -> Cursor {
        Cursor {
            chars: slice.chars(),
            len_remaining: slice.len(),
        }
    }
    /*fn eat_until(&mut self, until: char) {
        if let Some(index) = self.peek_position_of_first(until) {
            self.chars = self.chars.as_str()[index..].chars();
        } else {
            self.chars = "".chars();
        }
    }*/
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }
    pub(crate) fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    fn peek_nth(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }
    /*fn peek_position_of_first(&self, until: char) -> Option<usize> {
        memchr::memchr(until as u8, self.chars.as_str().as_bytes())
    }*/
    pub(crate) fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }
    pub(crate) fn eat_while(&mut self, f: impl Fn(Option<char>) -> bool) {
        while !self.is_eof() && f(self.peek()) {
            self.next_char();
        }
    }
    fn position_consumed(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn everything() {
        let mut cursor = Cursor::new("abcccc");
        assert_eq!(cursor.is_eof(), false);
        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.peek_nth(1), Some('b'));
        assert_eq!(cursor.next_char(), Some('a'));
        assert_eq!(cursor.peek(), Some('b'));
        assert_eq!(cursor.next_char(), Some('b'));
        cursor.eat_while(|c| c.unwrap() == 'c');
        assert_eq!(cursor.peek(), None);
        assert_eq!(cursor.is_eof(), true);
    }
}
