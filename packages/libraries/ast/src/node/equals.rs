use crate::trace::Trace;

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Equals {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl Equals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::Equals(Equals {
            left: left.into(),
            right: right.into(),
            trace: Trace::new(),
        })
    }
}
