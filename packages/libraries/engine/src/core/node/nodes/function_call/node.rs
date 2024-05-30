use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCall {
    pub target: Box<Node>,
    pub arguments: Vec<Node>,
}

impl FunctionCall {
    pub fn new<T: Into<Node>>(target: T, arguments: Vec<Node>) -> FunctionCall {
        FunctionCall {
            target: target.into().into(),
            arguments: arguments.into(),
        }
    }
}

impl From<FunctionCall> for Node {
    fn from(value: FunctionCall) -> Node {
        Node::FunctionCall(value)
    }
}

impl BaseNode for FunctionCall {}
