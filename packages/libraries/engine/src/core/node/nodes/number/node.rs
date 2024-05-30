use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn new(value: f64) -> Number {
        Number { value }
    }
}

impl From<Number> for Node {
    fn from(value: Number) -> Self {
        Node::Number(value)
    }
}

impl BaseNode for Number {}
