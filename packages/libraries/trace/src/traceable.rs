use crate::{IntoTrace, Span};

use super::trace::Trace;

pub trait Tracable: AsRef<Trace> {
    fn trace(&self) -> &Trace;
}

impl<T: AsRef<Trace>> Tracable for T {
    fn trace(&self) -> &Trace {
        self.as_ref()
    }
}

pub trait TracableMut: Sized {
    fn trace_mut(&mut self) -> &mut Trace;

    fn with_span(mut self, start: usize, end: usize) -> Self {
        self.trace_mut().push_span(Span::new(start, end));
        self
    }

    fn with_trace(mut self, trace: &Trace) -> Self {
        self.trace_mut().append_trace(&trace);
        self
    }

    fn with_tracable<T: IntoTrace>(mut self, tracable: T) -> Self {
        self.trace_mut().append_trace(&tracable.into_trace());
        self
    }
}

impl<T: AsMut<Trace>> TracableMut for T {
    fn trace_mut(&mut self) -> &mut Trace {
        self.as_mut()
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

impl<T1: Tracable, T2: Tracable, T3: Tracable> GetTraces for (T1, T2, T3) {
    fn traces(&self) -> Vec<Trace> {
        vec![
            self.0.trace().clone(),
            self.1.trace().clone(),
            self.2.trace().clone(),
        ]
    }
}

impl<T: Tracable> GetTraces for Vec<T> {
    fn traces(&self) -> Vec<Trace> {
        self.iter().map(|element| element.trace().clone()).collect()
    }
}

impl<T: Tracable> GetTraces for &Vec<T> {
    fn traces(&self) -> Vec<Trace> {
        self.iter().map(|element| element.trace().clone()).collect()
    }
}
