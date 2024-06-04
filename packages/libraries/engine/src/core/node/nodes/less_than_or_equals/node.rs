use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct LessThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl LessThanOrEquals {
    pub fn new(
        left: impl Into<Node>,
        right: impl Into<Node>,
    ) -> LessThanOrEquals {
        LessThanOrEquals {
            left: left.into().into(),
            right: right.into().into(),
        }
    }
}

impl From<LessThanOrEquals> for Node {
    fn from(value: LessThanOrEquals) -> Node {
        Node::LessThanOrEquals(value)
    }
}

impl BaseNode for LessThanOrEquals {}
