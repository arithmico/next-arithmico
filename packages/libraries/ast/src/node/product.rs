use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub elements: Vec<Node>,
}

impl Product {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Product(Self { elements })
    }
}
