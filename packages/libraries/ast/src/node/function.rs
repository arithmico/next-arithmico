use crate::trace::Trace;

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub arguments: Vec<String>,
    pub expression: Box<Node>,
    pub trace: Trace,
}

impl Function {
    pub fn new(arguments: Vec<String>, expression: Node) -> Node {
        Node::Function(Function {
            arguments,
            expression: Box::new(expression),
            trace: Trace::new(),
        })
    }
}
