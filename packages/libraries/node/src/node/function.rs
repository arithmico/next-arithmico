use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

mod function_signature;

pub use function_signature::*;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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
            trace: Trace::default(),
        })
    }
}

impl_node_traits!(Function);
