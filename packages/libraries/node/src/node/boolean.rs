use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub trace: Trace,
}

impl Boolean {
    pub fn new(value: bool) -> Node {
        Node::Boolean(Self {
            value,
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Boolean);
