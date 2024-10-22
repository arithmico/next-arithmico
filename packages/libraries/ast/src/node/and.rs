use trace::{GetTrace, GetTraceMut, Trace};

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct And {
    pub elements: Vec<Node>,
    pub trace: Trace,
}

impl And {
    pub fn new(values: Vec<Node>) -> Node {
        Node::And(And {
            elements: values,
            trace: Trace::new(),
        })
    }
}

impl GetTrace for And {
    fn trace(&self) -> &Trace {
        &self.trace
    }
}

impl GetTraceMut for And {
    fn trace_mut(&mut self) -> &mut Trace {
        &mut self.trace
    }
}
