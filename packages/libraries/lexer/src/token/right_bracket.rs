use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RightBracketToken {
    pub span: Span,
}

impl GetTokenKind for RightBracketToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::RightBracket
    }
}

impl GetStaticTokenKind for RightBracketToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::RightBracket
    }
}

impl GetTokenSpan for RightBracketToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for RightBracketToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::RightBracket(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
