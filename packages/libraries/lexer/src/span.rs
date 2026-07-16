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
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub(crate) fn extract_substr<'a, 'b>(&'a self, input: &'b str) -> &'b str {
        &input[self.from.byte_index..=self.to.byte_index]
    }
}

pub trait GetTokenSpan {
    fn get_span(&self) -> Span;
}
