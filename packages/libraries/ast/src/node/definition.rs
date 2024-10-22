use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Definition {
    pub symbol: String,
    pub expression: Box<Node>,
    pub trace: Trace,
}

impl Definition {
    pub fn new(symbol: impl Into<String>, expression: Node) -> Node {
        Node::Definition(Definition {
            symbol: symbol.into(),
            expression: expression.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Definition {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Definition {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
