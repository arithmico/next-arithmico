use crate::trace::Trace;

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Power {
    pub base: Box<Node>,
    pub exponent: Box<Node>,
    pub trace: Trace,
}

impl Power {
    pub fn new(base: Node, exponent: Node) -> Node {
        Node::Power(Self {
            base: Box::new(base),
            exponent: Box::new(exponent),
            trace: Trace::new(),
        })
    }
}
