use super::trace::Trace;

pub trait Traceable {
    fn trace_mut(&mut self) -> &mut Trace;
    fn trace(&self) -> &Trace;
}
