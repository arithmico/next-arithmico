use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Equals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Equals {
    pub fn new(left: impl Into<Node>, right: impl Into<Node>) -> Equals {
        Equals {
            left: left.into().into(),
            right: right.into().into(),
        }
    }
}

impl From<Equals> for Node {
    fn from(value: Equals) -> Node {
        Node::Equals(value)
    }
}

impl BaseNode for Equals {}
