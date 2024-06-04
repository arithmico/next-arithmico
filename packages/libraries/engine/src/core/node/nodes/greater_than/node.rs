use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct GreaterThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl GreaterThan {
    pub fn new(left: impl Into<Node>, right: impl Into<Node>) -> GreaterThan {
        GreaterThan {
            left: left.into().into(),
            right: right.into().into(),
        }
    }
}

impl From<GreaterThan> for Node {
    fn from(value: GreaterThan) -> Node {
        Node::GreaterThan(value)
    }
}

impl BaseNode for GreaterThan {}
