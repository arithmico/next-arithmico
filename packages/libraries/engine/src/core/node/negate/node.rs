use trace::Trace;

use crate::{core::Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Negate {
    pub value: Box<Node>,
    pub trace: Trace,
}

impl Negate {
    pub fn new(value: Node) -> Node {
        Node::Negate(Self {
            value: Box::new(value),
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Negate);
