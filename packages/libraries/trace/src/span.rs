#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn hull(&self, other: &Span) -> Span {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
