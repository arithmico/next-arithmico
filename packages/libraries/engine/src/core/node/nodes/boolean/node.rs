use crate::core::node::{node::BaseNode, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Boolean {
        Boolean { value }
    }
}

impl From<Boolean> for Node {
    fn from(value: Boolean) -> Node {
        Node::Boolean(value)
    }
}

impl BaseNode for Boolean {}
