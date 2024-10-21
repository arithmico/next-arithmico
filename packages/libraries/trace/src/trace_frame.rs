use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct TraceFrame {
    spans: Vec<Span>,
}

impl From<Span> for TraceFrame {
    fn from(value: Span) -> Self {
        Self { spans: vec![value] }
    }
}

impl TraceFrame {
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }

    pub fn hull_span(&self) -> Span {
        self.spans
            .iter()
            .cloned()
            .reduce(|left, right| left.hull(&right))
            .expect("hull span")
    }

    pub fn hull_trace_frame(&self, other: &TraceFrame) -> TraceFrame {
        self.hull_span().hull(&other.hull_span()).into()
    }

    pub fn push_span(&mut self, span: Span) {
        self.spans.push(span);
    }
}
