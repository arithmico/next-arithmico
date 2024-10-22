use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct LessThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl LessThan {
    pub fn new(left: Node, right: Node) -> Node {
        Node::LessThan(LessThan {
            left: left.into(),
            right: right.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for LessThan {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for LessThan {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
