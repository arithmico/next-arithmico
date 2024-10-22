use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Negate {
    pub value: Box<Node>,
    pub trace: Trace,
}

impl Negate {
    pub fn new(value: Node) -> Node {
        Node::Negate(Self {
            value: Box::new(value),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Negate {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Negate {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
