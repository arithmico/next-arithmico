use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    pub spans: Vec<Span>,
}

impl From<Span> for Trace {
    fn from(value: Span) -> Self {
        Self { spans: vec![value] }
    }
}

impl Trace {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    pub fn hull_span(&self) -> Span {
        self.spans
            .iter()
            .cloned()
            .reduce(|left, right| left.hull(&right))
            .expect("hull")
    }

    pub fn hull_trace(&self, other: &Trace) -> Trace {
        self.spans
            .iter()
            .chain(other.spans.iter())
            .cloned()
            .reduce(|left, right| left.hull(&right))
            .expect("hull")
            .into()
    }

    pub fn append_trace(&mut self, mut trace: Trace) {
        self.spans.append(&mut trace.spans);
    }
}
