use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Product {
    // TODO: make it fallible
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Product(Self {
            elements,
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Product);
