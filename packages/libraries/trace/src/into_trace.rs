use crate::{GetTraces, Trace};

pub trait IntoTrace {
    fn into_trace(self) -> Trace;
    fn into_hull_trace(self) -> Trace;
}

impl<T: GetTraces> IntoTrace for T {
    fn into_trace(self) -> Trace {
        todo!()
        /*self.traces()
        .into_iter()
        .reduce(|left, right| left.merge(&right))
        .unwrap_or(Trace::new())*/
    }

    fn into_hull_trace(self) -> Trace {
        todo!()
        /*self.traces()
        .into_iter()
        .reduce(|left, right| left.hull_trace(&right))
        .unwrap_or(Trace::new())*/
    }
}
