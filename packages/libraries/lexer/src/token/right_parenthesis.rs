use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RightParenthesisToken {
    pub span: Span,
}

impl GetTokenKind for RightParenthesisToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::RightParenthesis
    }
}

impl GetStaticTokenKind for RightParenthesisToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::RightParenthesis
    }
}

impl GetTokenSpan for RightParenthesisToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for RightParenthesisToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::RightParenthesis(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
