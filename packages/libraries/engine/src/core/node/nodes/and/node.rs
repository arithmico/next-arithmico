use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub values: Vec<Node>,
}

impl And {
    pub fn new(values: Vec<Node>) -> And {
        And { values }
    }
}

impl From<And> for Node {
    fn from(value: And) -> Node {
        Node::And(value)
    }
}

impl BaseNode for And {}
