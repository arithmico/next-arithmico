use trace::Trace;

use crate::{Node, impl_node_traits};

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

impl_node_traits!(LessThan);
