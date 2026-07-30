use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SlashToken {
    pub span: Span,
}

impl GetTokenKind for SlashToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Slash
    }
}

impl GetStaticTokenKind for SlashToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Slash
    }
}

impl GetTokenSpan for SlashToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for SlashToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Slash(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
