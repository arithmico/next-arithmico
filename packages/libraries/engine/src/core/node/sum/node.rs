use trace::Trace;

use crate::{core::Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Sum {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Sum(Self {
            elements,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Sum);
