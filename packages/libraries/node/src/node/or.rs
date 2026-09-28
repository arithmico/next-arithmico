use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Or {
    // TODO: make it fallible
    pub fn new(values: Vec<Node>) -> Node {
        Node::Or(Or {
            elements: values,
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Or);
