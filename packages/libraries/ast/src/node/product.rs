use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Product {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl Product {
    pub fn new(elements: Vec<Node>) -> Node {
        Node::Product(Self {
            elements,
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Product {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Product {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
