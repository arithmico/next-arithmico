use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Power {
    pub base: Box<Node>,
    pub exponent: Box<Node>,
}

impl Power {
    pub fn new(dividend: Node, divisor: Node) -> Node {
        Node::Power(Self {
            base: Box::new(dividend),
            exponent: Box::new(divisor),
        })
    }
}
