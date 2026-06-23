use trace::Trace;

use crate::{Node, impl_node_traits};

#[derive(PartialEq, Debug, Clone)]
pub struct Definition {
    pub symbol: String,
    pub expression: Box<Node>,
    pub trace: Trace,
}

impl Definition {
    pub fn new(symbol: impl Into<String>, expression: Node) -> Node {
        Node::Definition(Definition {
            symbol: symbol.into(),
            expression: expression.into(),
            trace: Trace::new(),
        })
    }
}

impl_node_traits!(Definition);
