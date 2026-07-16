use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LessThanOrEqualsToken {
    pub span: Span,
}

impl GetTokenKind for LessThanOrEqualsToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::LessThanOrEquals
    }
}

impl GetStaticTokenKind for LessThanOrEqualsToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::LessThanOrEquals
    }
}

impl GetTokenSpan for LessThanOrEqualsToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for LessThanOrEqualsToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::LessThanOrEquals(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
