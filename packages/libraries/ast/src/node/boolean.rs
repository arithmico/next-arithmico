use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub trace: Trace,
}

impl Boolean {
    pub fn new(value: bool) -> Node {
        Node::Boolean(Self {
            value,
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Boolean {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Boolean {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
