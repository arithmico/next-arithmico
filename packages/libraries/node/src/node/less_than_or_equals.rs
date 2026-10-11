use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LessThanOrEquals {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl LessThanOrEquals {
    pub fn new(left: Node, right: Node) -> Node {
        Node::LessThanOrEquals(LessThanOrEquals {
            left: left.into(),
            right: right.into(),
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(LessThanOrEquals);
