use trace::Trace;

use crate::{FunctionSignature, impl_node_traits};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub expression: Box<Node>,
    pub trace: Trace,
}

impl Function {
    pub fn new(signature: FunctionSignature, expression: Node) -> Node {
        Node::Function(Function {
            signature,
            expression: Box::new(expression),
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Function);
