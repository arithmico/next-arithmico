#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenKind {
    Identifier,
    Number,

    /// "true" or "false"
    Boolean,

    /// "("
    LeftParenthesis,

    /// ")"
    RightParenthesis,

    /// "["
    LeftBracket,

    /// "]"
    RightBracket,

    /// "+"
    Plus,

    /// "-"
    Minus,

    /// "*"
    Multiply,

    /// "/"
    Divide,

    /// "^"
    Caret,

    /// "," or ";" (german)
    Separator,

    /// "->"
    Arrow,

    /// ":="
    Define,

    /// "<"
    LessThan,

    /// "<="
    LessThanOrEquals,

    /// ">"
    GreaterThan,

    /// ">="
    GreaterThanOrEquals,

    /// "="
    Equals,
}

pub trait GetTokenKind {
    fn token_kind(&self) -> TokenKind;
}

pub trait GetStaticTokenKind {
    fn token_kind() -> TokenKind;
}
