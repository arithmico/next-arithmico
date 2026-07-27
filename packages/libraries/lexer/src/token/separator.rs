use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SeparatorToken {
    pub span: Span,
    pub content: String,
}

impl GetTokenKind for SeparatorToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Separator
    }
}

impl GetStaticTokenKind for SeparatorToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Separator
    }
}

impl GetTokenSpan for SeparatorToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for SeparatorToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Separator(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
