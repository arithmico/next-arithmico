use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OrToken {
    pub span: Span,
}

impl GetTokenKind for OrToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Or
    }
}

impl GetStaticTokenKind for OrToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Or
    }
}

impl GetTokenSpan for OrToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for OrToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Or(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
