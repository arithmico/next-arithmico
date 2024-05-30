use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub values: Vec<Node>,
}

impl Product {
    pub fn new(values: Vec<Node>) -> Product {
        Product { values }
    }
}

impl From<Product> for Node {
    fn from(value: Product) -> Node {
        Node::Product(value)
    }
}

impl BaseNode for Product {}
