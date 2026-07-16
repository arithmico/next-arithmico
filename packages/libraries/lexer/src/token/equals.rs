use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EqualsToken {
    pub span: Span,
}

impl GetTokenKind for EqualsToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Equals
    }
}

impl GetStaticTokenKind for EqualsToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Equals
    }
}

impl GetTokenSpan for EqualsToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for EqualsToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Equals(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
