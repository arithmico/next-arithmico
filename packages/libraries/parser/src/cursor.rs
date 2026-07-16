use lexer::Token;

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

    pub fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.position)
    }
}
