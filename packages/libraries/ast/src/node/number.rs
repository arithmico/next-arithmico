use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub trace: Trace,
}

impl Number {
    pub fn new(value: f64) -> Node {
        Node::Number(Self {
            value,
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Number {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Number {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
