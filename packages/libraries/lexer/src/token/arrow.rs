use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrowToken {
    pub span: Span,
}

impl GetTokenKind for ArrowToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Arrow
    }
}

impl GetStaticTokenKind for ArrowToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Arrow
    }
}

impl GetTokenSpan for ArrowToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for ArrowToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Arrow(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
