use lexer::{GetTokenKind, Token, TokenKind};

pub trait GetBindingPower {
    fn binding_power(&self) -> Option<u8>;
}

impl GetBindingPower for TokenKind {
    fn binding_power(&self) -> Option<u8> {
        match self {
            TokenKind::Identifier
            | TokenKind::Number
            | TokenKind::Boolean
            | TokenKind::LeftParenthesis
            | TokenKind::RightParenthesis
            | TokenKind::LeftBracket
            | TokenKind::Separator
            | TokenKind::Arrow
            | TokenKind::RightBracket => None,
            TokenKind::Define => Some(0),
            TokenKind::LessThanOrEquals => Some(1),
            TokenKind::LessThan => Some(1),
            TokenKind::GreaterThanOrEquals => Some(1),
            TokenKind::GreaterThan => Some(1),
            TokenKind::Equals => Some(1),
            TokenKind::Or => Some(2),
            TokenKind::And => Some(3),
            TokenKind::Plus => Some(4),
            TokenKind::Minus => Some(5),
            TokenKind::Multiply => Some(6),
            TokenKind::Divide => Some(7),
            TokenKind::Caret => Some(8),
        }
    }
}

impl GetBindingPower for Token {
    fn binding_power(&self) -> Option<u8> {
        self.token_kind().binding_power()
    }
}
