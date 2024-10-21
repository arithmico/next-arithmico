use trace::Trace;

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
