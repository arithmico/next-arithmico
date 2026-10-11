use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct GreaterThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl GreaterThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::GreaterThanOrEquals(GreaterThanOrEquals {
            left: left.into(),
            right: right.into(),
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(GreaterThanOrEquals);
