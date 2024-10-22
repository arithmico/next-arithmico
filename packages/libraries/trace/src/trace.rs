use crate::TraceFrame;

use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    frames: Vec<TraceFrame>,
}

impl From<TraceFrame> for Trace {
    fn from(value: TraceFrame) -> Self {
        Self {
            frames: vec![value],
        }
    }
}

impl From<Span> for Trace {
    fn from(value: Span) -> Self {
        Self {
            frames: vec![value.into()],
        }
    }
}

impl Trace {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn hull_span(&self) -> Span {
        self.frames
            .iter()
            .map(|frame| frame.hull_span())
            .reduce(|left, right| left.hull(&right))
            .expect("hull span")
    }

    pub fn hull_trace(&self, other: &Trace) -> Trace {
        self.frames
            .iter()
            .chain(other.frames.iter())
            .cloned()
            .reduce(|left, right| left.hull_trace_frame(&right))
            .expect("hull trace frame")
            .into()
    }

    pub fn append_trace(&mut self, trace: &Trace) {
        self.frames.append(&mut trace.frames.clone());
    }

    pub fn merge(&self, other: &Trace) -> Trace {
        let mut result = self.clone();
        result.append_trace(&other);
        result
    }

    pub fn push_frame(&mut self, frame: TraceFrame) {
        self.frames.push(frame);
    }

    pub fn push_span(&mut self, span: Span) {
        if let Some(frame) = self.frames.last_mut() {
            frame.push_span(span);
        } else {
            self.push_frame(span.into());
        }
    }

    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.push_span(Span::new(start, end));
        self
    }

    pub fn all_spans(&self) -> Vec<Span> {
        self.frames
            .iter()
            .flat_map(|frame| frame.spans().clone())
            .collect()
    }
}
