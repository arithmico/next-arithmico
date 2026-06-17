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
pub struct Position {
    pub byte_index: usize,
    pub char_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// from index
    pub from: Position,
    /// to index (inclusive)
    pub to: Position,
}

impl Span {
    pub(crate) fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub(crate) fn extract_substr<'a, 'b>(&'a self, input: &'b str) -> &'b str {
        &input[self.from.byte_index..=self.to.byte_index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub span: Span,
    pub content: TokenContent<'a>,
}
