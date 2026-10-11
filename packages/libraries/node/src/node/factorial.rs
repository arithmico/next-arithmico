use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Factorial {
    pub value: Box<Node>,
    pub trace: Trace,
}

impl Factorial {
    pub fn new(value: Node) -> Node {
        Node::Factorial(Self {
            value: Box::new(value),
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Factorial);
