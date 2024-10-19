use crate::trace::Trace;

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
