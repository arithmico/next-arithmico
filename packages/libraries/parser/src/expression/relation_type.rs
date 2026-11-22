use lexer::{GetTokenKind, Token, TokenKind};

use crate::Error;

pub enum RelationType {
    Equals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
}

impl RelationType {
    pub fn try_from_token(token: &Token) -> Result<Self, Error> {
        match token.token_kind() {
            TokenKind::Equals => Ok(RelationType::Equals),
            TokenKind::LessThan => Ok(RelationType::LessThan),
            TokenKind::LessThanOrEquals => Ok(RelationType::LessThanOrEquals),
            TokenKind::GreaterThan => Ok(RelationType::GreaterThan),
            TokenKind::GreaterThanOrEquals => {
                Ok(RelationType::GreaterThanOrEquals)
            }
            _ => Err(Error::UnexpectedToken {
                expected: vec![
                    TokenKind::Equals,
                    TokenKind::LessThan,
                    TokenKind::LessThanOrEquals,
                    TokenKind::GreaterThan,
                    TokenKind::GreaterThanOrEquals,
                ],
                actual: token.clone(),
            }),
        }
    }
}
