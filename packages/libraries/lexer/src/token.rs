use std::range::Range;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenContent<'a> {
    Identifier(&'a str),
    Number(f64),

    /// "true" or "false"
    Boolean(bool),

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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub span: Range<usize>,
    pub content: TokenContent<'a>,
}
