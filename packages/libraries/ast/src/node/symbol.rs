use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub trace: Trace,
}

impl Symbol {
    pub fn new(name: &str) -> Node {
        Node::Symbol(Self {
            name: name.to_string(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for Symbol {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for Symbol {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
