use trace::{Tracable, TracableMut, Trace};

use super::Node;

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

impl TracableMut for FunctionCall {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for FunctionCall {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
