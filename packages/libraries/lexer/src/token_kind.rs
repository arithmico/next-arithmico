#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenKind {
    /// any variable or function name
    Identifier,

    /// any number
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

    /// "&"
    And,

    /// "|"
    Or,
}

pub trait GetTokenKind {
    fn token_kind(&self) -> TokenKind;
}

pub trait GetStaticTokenKind {
    fn token_kind() -> TokenKind;
}
