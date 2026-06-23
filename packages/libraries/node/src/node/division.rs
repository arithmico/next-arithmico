use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Division {
    pub dividend: Box<Node>,
    pub divisor: Box<Node>,
    pub trace: Trace,
}

impl Division {
    pub fn new(dividend: Node, divisor: Node) -> Node {
        Node::Division(Self {
            dividend: Box::new(dividend),
            divisor: Box::new(divisor),
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Division);
