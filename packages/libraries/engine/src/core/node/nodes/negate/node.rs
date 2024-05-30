use crate::core::node::{node::BaseNode, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Negate {
    pub value: Box<Node>,
}

impl Negate {
    pub fn new<T: Into<Node>>(value: T) -> Negate {
        Negate {
            value: value.into().into(),
        }
    }
}

impl From<Negate> for Node {
    fn from(value: Negate) -> Node {
        Node::Negate(value)
    }
}

impl BaseNode for Negate {}
