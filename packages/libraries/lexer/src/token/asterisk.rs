use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AsteriskToken {
    pub span: Span,
}

impl GetTokenKind for AsteriskToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Asterisk
    }
}

impl GetStaticTokenKind for AsteriskToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Asterisk
    }
}

impl GetTokenSpan for AsteriskToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for AsteriskToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Asterisk(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
