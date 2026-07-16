use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MinusToken {
    pub span: Span,
}

impl GetTokenKind for MinusToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Minus
    }
}

impl GetStaticTokenKind for MinusToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Minus
    }
}

impl GetTokenSpan for MinusToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for MinusToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Minus(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
