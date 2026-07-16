use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GreaterThanToken {
    pub span: Span,
}

impl GetTokenKind for GreaterThanToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::GreaterThan
    }
}

impl GetStaticTokenKind for GreaterThanToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::GreaterThan
    }
}

impl GetTokenSpan for GreaterThanToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for GreaterThanToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::GreaterThan(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
