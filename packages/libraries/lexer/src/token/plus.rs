use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PlusToken {
    pub span: Span,
}

impl GetTokenKind for PlusToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Plus
    }
}

impl GetStaticTokenKind for PlusToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Plus
    }
}

impl GetTokenSpan for PlusToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for PlusToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Plus(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
