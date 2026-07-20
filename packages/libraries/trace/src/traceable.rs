use lexer::Span;

use crate::{IntoTrace, frame::Frame};

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

    fn with_optional_span(self, span: Option<Span>) -> Self {
        if let Some(span) = span {
            self.with_span(span)
        } else {
            self
        }
    }

    fn with_trace(mut self, trace: &Trace) -> Self {
        let t = self.trace_mut();
        t.compact();
        match (t.last_mut(), trace.last().cloned()) {
            (_, None) => (),
            (None, Some(frame)) => self.trace_mut().push(frame),
            (Some(last), Some(frame)) => last.merge(frame),
        }
        self
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

    fn with_tracable<T: IntoTrace>(self, tracable: T) -> Self {
        self.with_trace(&tracable.into_trace())
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
