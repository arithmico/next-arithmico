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
        Some(self.cmp(other))
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

    #[cfg(feature = "test_utils")]
    pub fn new_between(from: usize, to: usize) -> Self {
        Self {
            from: Position::new_at(from),
            to: Position::new_at(to),
        }
    }

    pub fn extract_str_from<'b>(&self, input: &'b str) -> &'b str {
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

    pub fn len_bytes(&self) -> usize {
        self.from.byte_index.abs_diff(self.to.byte_index) + 1
    }

    pub fn len_chars(&self) -> usize {
        self.from.char_index.abs_diff(self.to.char_index) + 1
    }

    pub fn from_byte_offset(&self) -> usize {
        self.from.byte_index
    }

    pub fn to_byte_offset(&self) -> usize {
        self.to.byte_index
    }
}

pub trait GetTokenSpan {
    fn get_span(&self) -> Span;
}
