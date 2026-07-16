use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CaretToken {
    pub span: Span,
}

impl GetTokenKind for CaretToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Caret
    }
}

impl GetStaticTokenKind for CaretToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Caret
    }
}

impl GetTokenSpan for CaretToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for CaretToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Caret(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
