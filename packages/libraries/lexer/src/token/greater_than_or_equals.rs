use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GreaterThanOrEqualsToken {
    pub span: Span,
}

impl GetTokenKind for GreaterThanOrEqualsToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::GreaterThanOrEquals
    }
}

impl GetStaticTokenKind for GreaterThanOrEqualsToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::GreaterThanOrEquals
    }
}

impl GetTokenSpan for GreaterThanOrEqualsToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for GreaterThanOrEqualsToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::GreaterThanOrEquals(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
