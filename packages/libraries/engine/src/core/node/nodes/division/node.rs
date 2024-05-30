use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Division {
    pub dividend: Box<Node>,
    pub divisor: Box<Node>,
}

impl Division {
    pub fn new(
        dividend: impl Into<Node>,
        divisor: impl Into<Node>,
    ) -> Division {
        Division {
            dividend: dividend.into().into(),
            divisor: divisor.into().into(),
        }
    }
}

impl From<Division> for Node {
    fn from(value: Division) -> Node {
        Node::Division(value)
    }
}

impl BaseNode for Division {}
