use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LessThan {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub trace: Trace,
}

impl LessThan {
    pub fn new(left: Node, right: Node) -> Node {
        Node::LessThan(LessThan {
            left: left.into(),
            right: right.into(),
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(LessThan);
