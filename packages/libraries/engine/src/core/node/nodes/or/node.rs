use crate::core::node::{node::BaseNode, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub values: Vec<Node>,
}

impl Or {
    pub fn new(values: Vec<Node>) -> Or {
        Or { values }
    }
}

impl From<Or> for Node {
    fn from(value: Or) -> Node {
        Node::Or(value)
    }
}

impl BaseNode for Or {}
