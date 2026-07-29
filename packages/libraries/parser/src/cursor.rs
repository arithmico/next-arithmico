use lexer::{GetTokenKind, GetTokenSpan, Position, Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn next(&mut self) -> Option<&'a Token> {
        if self.is_eof() {
            return None;
        }
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    pub fn current(&self) -> Option<&'a Token> {
        self.tokens.get(self.position.checked_sub(1)?)
    }

    pub fn current_token_kind(&self) -> Option<TokenKind> {
        self.current().map(|token| token.token_kind())
    }

    pub fn current_position(&self) -> Option<Position> {
        self.current().map(|token| token.get_span().from)
    }

    pub fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.position)
    }

    pub fn peek_token_kind(&self) -> Option<TokenKind> {
        self.peek().map(|token| token.token_kind())
    }

    pub fn peek_position(&self) -> Option<Position> {
        self.peek().map(|token| token.get_span().from)
    }
}
