use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Negate {
    pub value: Box<Node>,
}

impl Negate {
    pub fn new(value: Node) -> Node {
        Node::Negate(Self {
            value: Box::new(value),
        })
    }
}
