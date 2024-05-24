use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Power {
    pub base: Box<Node>,
    pub exponent: Box<Node>,
}

impl Power {
    pub fn new<L: Into<Node>, R: Into<Node>>(base: L, exponent: R) -> Power {
        Power {
            base: base.into().into(),
            exponent: exponent.into().into(),
        }
    }
}
