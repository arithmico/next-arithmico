use serde::{Deserialize, Serialize};

use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightBraceToken {
    pub span: Span,
}

impl GetTokenKind for RightBraceToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::RightBrace
    }
}

impl GetStaticTokenKind for RightBraceToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::RightBrace
    }
}

impl GetTokenSpan for RightBraceToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for RightBraceToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::RightBrace(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
