use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Tensor {
    pub elements: Vec<Node>,
}

impl Tensor {
    pub fn new(elements: Vec<Node>) -> Tensor {
        Tensor { elements }
    }
}
