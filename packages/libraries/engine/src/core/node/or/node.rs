use trace::Trace;

use crate::{impl_node_traits, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Or {
    pub fn new(values: Vec<Node>) -> Node {
        Node::Or(Or {
            elements: values,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Or);
