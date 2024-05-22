use crate::core::node::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub values: Vec<Node>,
}

impl Sum {
    pub fn new(values: Vec<Node>) -> Sum {
        Sum { values }
    }
}
