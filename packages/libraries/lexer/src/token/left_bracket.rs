use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LeftBracketToken {
    pub span: Span,
}

impl GetTokenKind for LeftBracketToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::LeftBracket
    }
}

impl GetStaticTokenKind for LeftBracketToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::LeftBracket
    }
}

impl GetTokenSpan for LeftBracketToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for LeftBracketToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::LeftBracket(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
