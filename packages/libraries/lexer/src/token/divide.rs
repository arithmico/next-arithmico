use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DivideToken {
    pub span: Span,
}

impl GetTokenKind for DivideToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Divide
    }
}

impl GetStaticTokenKind for DivideToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Divide
    }
}

impl GetTokenSpan for DivideToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for DivideToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Divide(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
