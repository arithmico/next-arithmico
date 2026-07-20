use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// from index
    pub from: Position,
    /// to index (inclusive)
    pub to: Position,
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.from.cmp(&other.from) {
            std::cmp::Ordering::Greater => self.to.cmp(&other.to),
            ordering => ordering,
        }
    }
}

impl Span {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub fn extract_str_from<'a, 'b>(&'a self, input: &'b str) -> &'b str {
        &input[self.from.byte_index..=self.to.byte_index]
    }

    pub fn hull(&self, other: &Self) -> Self {
        let min = self.from.min(other.from);
        let max = self.to.max(other.to);
        Self { from: min, to: max }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }
}

pub trait GetTokenSpan {
    fn get_span(&self) -> Span;
}
