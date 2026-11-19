use serde::{Deserialize, Serialize};

use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackslashToken {
    pub span: Span,
}

impl GetTokenKind for BackslashToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Backslash
    }
}

impl GetStaticTokenKind for BackslashToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Backslash
    }
}

impl GetTokenSpan for BackslashToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for BackslashToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Backslash(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
