use crate::{
    DowncastToken, GetStaticTokenKind, GetTokenKind, GetTokenSpan, Span,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MultiplyToken {
    pub span: Span,
}

impl GetTokenKind for MultiplyToken {
    fn token_kind(&self) -> crate::TokenKind {
        crate::TokenKind::Multiply
    }
}

impl GetStaticTokenKind for MultiplyToken {
    fn token_kind() -> crate::TokenKind {
        crate::TokenKind::Multiply
    }
}

impl GetTokenSpan for MultiplyToken {
    fn get_span(&self) -> Span {
        self.span
    }
}

impl DowncastToken for MultiplyToken {
    fn downcast(token: &super::Token) -> Option<&Self> {
        if let crate::Token::Multiply(token) = token {
            Some(token)
        } else {
            None
        }
    }
}
