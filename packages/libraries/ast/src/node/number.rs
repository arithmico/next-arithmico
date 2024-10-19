use crate::trace::Trace;

use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub trace: Trace,
}

impl Number {
    pub fn new(value: f64) -> Node {
        Node::Number(Self {
            value,
            trace: Trace::new(),
        })
    }
}
