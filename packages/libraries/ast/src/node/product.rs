use trace::Trace;

use super::Node;

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
