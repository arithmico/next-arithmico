use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NumberToken {
    pub value: f64,
    pub span: Span,
}

impl GetTokenKind for NumberToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Number
    }
}

impl GetStaticTokenKind for NumberToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Number
    }
}

impl GetTokenSpan for NumberToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for NumberToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Number(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
