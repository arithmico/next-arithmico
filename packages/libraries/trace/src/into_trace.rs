use crate::{GetTraces, Trace};

pub trait IntoTrace {
    fn into_trace(self) -> Trace;
}

impl<T: GetTraces> IntoTrace for T {
    fn into_trace(self) -> Trace {
        self.traces()
            .into_iter()
            .reduce(|left, right| left.merge(&right))
            .expect("trace")
    }
}
