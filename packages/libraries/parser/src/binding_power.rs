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
            TokenKind::Plus => Some(2),
            TokenKind::Minus => Some(3),
            TokenKind::Multiply => Some(4),
            TokenKind::Divide => Some(5),
            TokenKind::Caret => Some(6),
        }
    }
}

impl GetBindingPower for Token {
    fn binding_power(&self) -> Option<u8> {
        self.token_kind().binding_power()
    }
}
