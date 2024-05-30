use crate::core::node::{node::BaseNode, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub values: Vec<Node>,
}

impl Sum {
    pub fn new(values: Vec<Node>) -> Sum {
        Sum { values }
    }
}

impl From<Sum> for Node {
    fn from(value: Sum) -> Node {
        Node::Sum(value)
    }
}

impl BaseNode for Sum {}
