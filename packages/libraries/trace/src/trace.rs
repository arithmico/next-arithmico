use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    spans: Vec<Span>,
}

impl From<Span> for Trace {
    fn from(value: Span) -> Self {
        Self {
            spans: vec![value.into()],
        }
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
            .expect("hull span")
    }

    pub fn hull_trace(&self, other: &Trace) -> Trace {
        self.spans
            .iter()
            .chain(other.spans.iter())
            .cloned()
            .reduce(|left, right| left.hull(&right))
            .expect("hull span")
            .into()
    }

    pub fn append_trace(&mut self, trace: &Trace) {
        self.spans.append(&mut trace.spans.clone());
        self.merge_spans();
    }

    /// merges all spans in the trace in O(n * log(n))
    pub fn merge_spans(&mut self) {
        let mut sorted_spans: Vec<Span> = self.spans.clone();
        sorted_spans.sort_by(|left, right| left.start().cmp(&right.start()));

        let spans = if sorted_spans.len() < 1 {
            sorted_spans
        } else {
            let mut result = vec![sorted_spans.get(0).expect("span").clone()];

            for current in sorted_spans.into_iter().skip(1) {
                let previous = result.last_mut().expect("previous span");
                if current.start() <= previous.end() {
                    let end = previous.end().max(current.end());
                    previous.set_end(end);
                } else {
                    result.push(current);
                }
            }

            result
        };

        self.spans = spans;
    }

    /// merge two traces and their spans in O(n * log(n))
    pub fn merge(&self, other: &Trace) -> Trace {
        let spans = self
            .spans
            .iter()
            .chain(other.spans.iter())
            .cloned()
            .collect::<Vec<_>>();
        let mut trace = Trace { spans };
        trace.merge_spans();
        trace
    }

    pub fn push_span(&mut self, span: Span) {
        self.spans.push(span);
        self.merge_spans();
    }

    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.push_span(Span::new(start, end));
        self
    }

    pub fn spans(&self) -> Vec<Span> {
        self.spans.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn merge_spans() {
        let trace =
            Trace::new().with_span(0, 0).with_span(2, 2).with_span(0, 2);
        assert_eq!(trace, Trace::new().with_span(0, 2))
    }

    #[test]
    pub fn merge_traces() {
        let trace_1 = Trace::new().with_span(0, 0).with_span(2, 2);
        let trace_2 = Trace::new().with_span(0, 2);
        assert_eq!(trace_1.merge(&trace_2), Trace::new().with_span(0, 2))
    }
}
