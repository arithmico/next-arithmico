use trace::Trace;

use crate::{impl_node_traits, Node};

mod function_signature;

pub use function_signature::*;

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
