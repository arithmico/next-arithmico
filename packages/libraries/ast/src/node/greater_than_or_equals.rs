use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct GreaterThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl GreaterThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::GreaterThanOrEquals(GreaterThanOrEquals {
            left: left.into(),
            right: right.into(),
        })
    }
}
