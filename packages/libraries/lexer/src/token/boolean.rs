use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanToken {
    pub value: bool,
    pub span: Span,
}

impl GetTokenKind for BooleanToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Boolean
    }
}

impl GetStaticTokenKind for BooleanToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Boolean
    }
}

impl GetTokenSpan for BooleanToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for BooleanToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Boolean(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
