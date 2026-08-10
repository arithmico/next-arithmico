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
            TokenKind::Or => Some(1),
            TokenKind::And => Some(2),
            TokenKind::LessThanOrEquals => Some(3),
            TokenKind::LessThan => Some(3),
            TokenKind::GreaterThanOrEquals => Some(3),
            TokenKind::GreaterThan => Some(3),
            TokenKind::Equals => Some(3),
            TokenKind::Plus => Some(4),
            TokenKind::Minus => Some(5),
            TokenKind::Asterisk => Some(6),
            TokenKind::Slash => Some(7),
            TokenKind::Caret => Some(8),
            TokenKind::ExclamationMark => Some(9),
        }
    }
}

impl GetBindingPower for Token {
    fn binding_power(&self) -> Option<u8> {
        self.token_kind().binding_power()
    }
}
