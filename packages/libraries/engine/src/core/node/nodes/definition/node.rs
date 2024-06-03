use crate::core::node::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Definition {
    pub symbol: String,
    pub expression: Box<Node>,
}

impl Definition {
    pub fn new(
        symbol: impl Into<String>,
        expression: impl Into<Node>,
    ) -> Definition {
        Definition {
            symbol: symbol.into(),
            expression: expression.into().into(),
        }
    }
}

impl From<Definition> for Node {
    fn from(value: Definition) -> Node {
        Node::Definition(value)
    }
}

impl BaseNode for Definition {}
