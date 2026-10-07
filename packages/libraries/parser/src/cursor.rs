use lexer::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Position,
    Token, TokenKind,
};

use crate::Error;

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

    pub fn next_if<T: GetStaticTokenKind + DowncastToken>(
        &mut self,
    ) -> Option<&'a T> {
        if self.peek_token_kind() == Some(T::token_kind()) {
            self.next().and_then(|token| T::downcast(token))
        } else {
            None
        }
    }

    pub fn expect<T: GetStaticTokenKind + DowncastToken>(
        &mut self,
    ) -> Result<&'a T, Error> {
        if let Some(token) = self.next() {
            if let Some(token) = T::downcast(token) {
                Ok(token)
            } else {
                Err(Error::UnexpectedToken {
                    expected: vec![T::token_kind()],
                    actual: token.clone(),
                })
            }
        } else {
            Err(Error::UnexpectedEndOfInput)
        }
    }

    pub fn expect_one_of(
        &mut self,
        token_kinds: &[TokenKind],
    ) -> Result<&'a Token, Error> {
        if let Some(token) = self.next() {
            if token_kinds.contains(&token.token_kind()) {
                Ok(token)
            } else {
                Err(Error::UnexpectedToken {
                    expected: token_kinds.to_vec(),
                    actual: token.clone(),
                })
            }
        } else {
            Err(Error::UnexpectedEndOfInput)
        }
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

    pub fn current_position_or_err(&self) -> Result<Position, Error> {
        self.current_position().ok_or(Error::UnexpectedEndOfInput)
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

    pub fn peek_position_or_err(&self) -> Result<Position, Error> {
        self.peek_position().ok_or(Error::UnexpectedEndOfInput)
    }
}
