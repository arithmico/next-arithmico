use trace::{Tracable, TracableMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct HostFunction {
    pub name: String,
    pub trace: Trace,
}

impl HostFunction {
    pub fn new<T: Into<String>>(name: T) -> Node {
        Node::HostFunction(HostFunction {
            name: name.into(),
            trace: Trace::new(),
        })
    }
}

impl TracableMut for HostFunction {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}

impl Tracable for HostFunction {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}
