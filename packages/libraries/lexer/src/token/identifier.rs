use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierToken {
    pub name: String,
    pub span: Span,
}

impl GetTokenKind for IdentifierToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Identifier
    }
}

impl GetStaticTokenKind for IdentifierToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Identifier
    }
}

impl GetTokenSpan for IdentifierToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for IdentifierToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Identifier(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
