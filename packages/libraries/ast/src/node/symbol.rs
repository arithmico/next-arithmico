use crate::trace::Trace;

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
