use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct LessThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl LessThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::LessThanOrEquals(LessThanOrEquals {
            left: left.into(),
            right: right.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for LessThanOrEquals {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for LessThanOrEquals {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
