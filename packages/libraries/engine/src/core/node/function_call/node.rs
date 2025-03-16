use trace::Trace;

use crate::{core::Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub target: Box<Node>,
    pub arguments: Vec<Node>,
    pub trace: Trace,
}

impl FunctionCall {
    pub fn new(target: Node, arguments: Vec<Node>) -> Node {
        Node::FunctionCall(FunctionCall {
            target: target.into(),
            arguments,
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(FunctionCall);
