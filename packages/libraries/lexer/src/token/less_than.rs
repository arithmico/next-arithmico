use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LessThanToken {
    pub span: Span,
}

impl GetTokenKind for LessThanToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::LessThan
    }
}

impl GetStaticTokenKind for LessThanToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::LessThan
    }
}

impl GetTokenSpan for LessThanToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for LessThanToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::LessThan(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
