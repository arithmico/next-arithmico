use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct GreaterThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl GreaterThanOrEquals {
    pub fn new(
        left: impl Into<Node>,
        right: impl Into<Node>,
    ) -> GreaterThanOrEquals {
        GreaterThanOrEquals {
            left: left.into().into(),
            right: right.into().into(),
        }
    }
}

impl From<GreaterThanOrEquals> for Node {
    fn from(value: GreaterThanOrEquals) -> Node {
        Node::GreaterThanOrEquals(value)
    }
}

impl BaseNode for GreaterThanOrEquals {}
