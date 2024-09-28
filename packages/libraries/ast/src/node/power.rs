use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Power {
    pub base: Box<Node>,
    pub exponent: Box<Node>,
}

impl Power {
    pub fn new(base: Node, exponent: Node) -> Node {
        Node::Power(Self {
            base: Box::new(base),
            exponent: Box::new(exponent),
        })
    }
}
