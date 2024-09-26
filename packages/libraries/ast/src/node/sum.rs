use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub elements: Vec<Node>,
}

impl Sum {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Sum(Self { elements })
    }
}
