use trace::Trace;

use crate::{impl_node_traits, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub trace: Trace,
}

impl Boolean {
    pub fn new(value: bool) -> Node {
        Node::Boolean(Self {
            value,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Boolean);
