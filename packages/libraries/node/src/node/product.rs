use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Product {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Product(Self {
            elements,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Product);
