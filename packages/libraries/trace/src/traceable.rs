use lexer::Span;

use crate::frame::Frame;

use super::trace::Trace;

pub trait Tracable: AsRef<Trace> {
    fn trace(&self) -> &Trace;

    fn hull(&self) -> Option<Span> {
        self.trace().last().map(|frame| frame.hull())
    }
}

impl<T: AsRef<Trace>> Tracable for T {
    fn trace(&self) -> &Trace {
        self.as_ref()
    }
}

impl AsRef<Trace> for Trace {
    fn as_ref(&self) -> &Trace {
        &self
    }
}

pub trait TracableMut: Sized {
    fn trace_mut(&mut self) -> &mut Trace;

    /// append a span to the last trace frame
    fn with_span(mut self, span: Span) -> Self {
        let trace = self.trace_mut();
        match trace.last_mut() {
            Some(frame) => frame.push(span),
            None => trace.push(Frame::new(span)),
        }
        self
    }

    fn with_new_frame(mut self, span: Span) -> Self {
        let trace = self.trace_mut();
        trace.push(Frame::new(span));
        self
    }

    fn with_optional_new_frame(mut self, span: Option<Span>) -> Self {
        if let Some(span) = span {
            self.trace_mut().push(Frame::new(span));
        }
        self
    }

    fn with_optional_span(self, span: Option<Span>) -> Self {
        if let Some(span) = span {
            self.with_span(span)
        } else {
            self
        }
    }

    /// reduce the trace to a single hull span if possible
    fn only_hull(mut self) -> Self {
        let trace = self.trace_mut();
        trace.compact();
        if let Some(frame) = trace.last_mut() {
            if !frame.is_hull() {
                let hull = frame.hull();
                let hull_frame = Frame::from(hull);
                *frame = hull_frame;
            }
        }
        self
    }
}

impl<T: AsMut<Trace>> TracableMut for T {
    fn trace_mut(&mut self) -> &mut Trace {
        self.as_mut()
    }
}
