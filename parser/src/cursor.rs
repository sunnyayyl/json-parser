use lexer::{IntoIter, Lexer, LexerToken};

pub(crate) struct TokenCursor<'a> {
    tokens: IntoIter<'a>,
}
impl<'a> TokenCursor<'a> {
    pub(crate) fn new(lexer: Lexer<'a>) -> TokenCursor<'a> {
        Self {
            tokens: lexer.into_iter(),
        }
    }
    pub(crate) fn is_eof(&self) -> bool {
        self.tokens.eof
    }
    pub(crate) fn peek(&self) -> Option<LexerToken> {
        self.tokens.clone().next()
    }
    pub(crate) fn peek_nth(&self, n: usize) -> Option<LexerToken> {
        self.tokens.clone().nth(n)
    }
    pub(crate) fn next_token(&mut self) -> Option<LexerToken> {
        self.tokens.next()
    }

}
