use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub values: Vec<Node>,
}

impl Or {
    pub fn new(values: Vec<Node>) -> Node {
        Node::Or(Or { values })
    }
}
