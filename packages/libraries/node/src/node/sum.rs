use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Sum {
    // TODO: make it fallible
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Sum {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Sum(Self {
            elements,
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Sum);
