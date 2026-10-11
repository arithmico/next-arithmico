use serde::{Deserialize, Serialize};

use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AndToken {
    pub span: Span,
}

impl GetTokenKind for AndToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::And
    }
}

impl GetStaticTokenKind for AndToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::And
    }
}

impl GetTokenSpan for AndToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for AndToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::And(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
