use trace::Trace;

use crate::{core::Node, impl_node_traits};

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

impl_node_traits!(GreaterThanOrEquals);
