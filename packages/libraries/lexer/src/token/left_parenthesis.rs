use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LeftParenthesisToken {
    pub span: Span,
}

impl GetTokenKind for LeftParenthesisToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::LeftParenthesis
    }
}

impl GetStaticTokenKind for LeftParenthesisToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::LeftParenthesis
    }
}

impl GetTokenSpan for LeftParenthesisToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for LeftParenthesisToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::LeftParenthesis(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
