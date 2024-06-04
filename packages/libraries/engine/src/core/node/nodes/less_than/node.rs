use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LessThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl LessThan {
    pub fn new(left: impl Into<Node>, right: impl Into<Node>) -> LessThan {
        LessThan {
            left: left.into().into(),
            right: right.into().into(),
        }
    }
}

impl From<LessThan> for Node {
    fn from(value: LessThan) -> Node {
        Node::LessThan(value)
    }
}

impl BaseNode for LessThan {}
