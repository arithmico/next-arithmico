use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Number {
    pub value: f64,
    pub trace: Trace,
}

impl Number {
    pub fn new_node(value: f64) -> Node {
        Self::new(value).into()
    }

    pub fn new(value: f64) -> Self {
        Self {
            value,
            trace: Trace::new(),
        }
    }
}

impl_node_traits!(Number);
