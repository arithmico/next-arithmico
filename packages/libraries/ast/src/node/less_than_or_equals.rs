use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct LessThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl LessThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::LessThanOrEquals(LessThanOrEquals {
            left: left.into(),
            right: right.into(),
        })
    }
}
