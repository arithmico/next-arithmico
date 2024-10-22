use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Division {
    pub dividend: Box<Node>,
    pub divisor: Box<Node>,
    pub trace: Trace,
}

impl Division {
    pub fn new(dividend: Node, divisor: Node) -> Node {
        Node::Division(Self {
            dividend: Box::new(dividend),
            divisor: Box::new(divisor),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Division {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Division {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
