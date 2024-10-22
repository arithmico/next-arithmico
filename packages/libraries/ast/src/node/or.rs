use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Or {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Or {
    pub fn new(values: Vec<Node>) -> Node {
        Node::Or(Or {
            elements: values,
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Or {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Or {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
