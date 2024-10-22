use crate::Span;

use super::trace::Trace;

pub trait Tracable {
    fn trace(&self) -> &Trace;
}

impl<T: Tracable> Tracable for &T {
    fn trace(&self) -> &Trace {
        (*self).trace()
    }
}

pub trait TracableMut: Sized {
    fn trace_mut(&mut self) -> &mut Trace;

    fn with_span(mut self, start: usize, end: usize) -> Self {
        self.trace_mut().push_span(Span::new(start, end));
        self
    }

    fn with_trace(mut self, trace: Trace) -> Self {
        self.trace_mut().append_trace(&trace);
        self
    }
}

pub trait GetTraces {
    fn traces(&self) -> Vec<Trace>;
}

impl<T: Tracable> GetTraces for T {
    fn traces(&self) -> Vec<Trace> {
        vec![self.trace().clone()]
    }
}

impl<T1: Tracable, T2: Tracable> GetTraces for (T1, T2) {
    fn traces(&self) -> Vec<Trace> {
        vec![self.0.trace().clone(), self.1.trace().clone()]
    }
}
