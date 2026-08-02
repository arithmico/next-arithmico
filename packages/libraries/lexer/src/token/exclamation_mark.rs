use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExclamationMarkToken {
    pub span: Span,
}

impl GetTokenKind for ExclamationMarkToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::ExclamationMark
    }
}

impl GetStaticTokenKind for ExclamationMarkToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::ExclamationMark
    }
}

impl GetTokenSpan for ExclamationMarkToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for ExclamationMarkToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::ExclamationMark(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
