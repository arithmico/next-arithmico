use crate::core::node::{node::BaseNode, Node};

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new<T: Into<String>>(name: T) -> Symbol {
        Symbol { name: name.into() }
    }
}

impl From<Symbol> for Node {
    fn from(value: Symbol) -> Node {
        Node::Symbol(value)
    }
}

impl BaseNode for Node {}
