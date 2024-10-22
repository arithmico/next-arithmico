use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct GreaterThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl GreaterThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::GreaterThanOrEquals(GreaterThanOrEquals {
            left: left.into(),
            right: right.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for GreaterThanOrEquals {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for GreaterThanOrEquals {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
