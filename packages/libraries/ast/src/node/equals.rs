use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Equals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Equals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::Equals(Equals {
            left: left.into(),
            right: right.into(),
        })
    }
}
