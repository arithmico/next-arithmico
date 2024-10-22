use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct GreaterThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl GreaterThan {
    pub fn new(left: Node, right: Node) -> Node {
        Node::GreaterThan(GreaterThan {
            left: left.into(),
            right: right.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for GreaterThan {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for GreaterThan {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
