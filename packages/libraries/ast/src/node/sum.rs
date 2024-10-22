use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Sum {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Sum {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Sum(Self {
            elements,
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Sum {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Sum {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
