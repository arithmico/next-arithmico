use crate::core::node::Node;

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
