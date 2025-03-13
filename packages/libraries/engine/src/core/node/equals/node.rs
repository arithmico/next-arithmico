use trace::Trace;

use crate::{impl_node_traits, Node};

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

impl_node_traits!(Equals);
