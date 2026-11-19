use serde::{Deserialize, Serialize};

use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeftBraceToken {
    pub span: Span,
}

impl GetTokenKind for LeftBraceToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::LeftBrace
    }
}

impl GetStaticTokenKind for LeftBraceToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::LeftBrace
    }
}

impl GetTokenSpan for LeftBraceToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for LeftBraceToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::LeftBrace(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
