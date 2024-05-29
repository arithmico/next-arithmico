use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub values: Vec<Node>,
}

impl Or {
    pub fn new(values: Vec<Node>) -> Or {
        Or { values }
    }
}
