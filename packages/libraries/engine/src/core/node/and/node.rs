use trace::Trace;

use crate::{core::Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl And {
    pub fn new(values: Vec<Node>) -> Node {
        Node::And(And {
            elements: values,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(And);
