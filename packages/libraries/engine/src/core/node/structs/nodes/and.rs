use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub values: Vec<Node>,
}

impl And {
    pub fn new(values: Vec<Node>) -> And {
        And { values }
    }
}
