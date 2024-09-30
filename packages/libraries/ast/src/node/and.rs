use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub elements: Vec<Node>,
}

impl And {
    pub fn new(values: Vec<Node>) -> Node {
        Node::And(And { elements: values })
    }
}
