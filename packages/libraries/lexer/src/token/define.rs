use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DefineToken {
    pub span: Span,
}

impl GetTokenKind for DefineToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Define
    }
}

impl GetStaticTokenKind for DefineToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Define
    }
}

impl GetTokenSpan for DefineToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for DefineToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Define(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
