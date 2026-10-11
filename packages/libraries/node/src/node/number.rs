use serde::{Deserialize, Serialize};
use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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
            trace: Trace::default(),
        }
    }
}

impl_node_traits!(Number);
