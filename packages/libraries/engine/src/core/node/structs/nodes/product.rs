use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub values: Vec<Node>,
}

impl Product {
    pub fn new(values: Vec<Node>) -> Product {
        Product { values }
    }
}
