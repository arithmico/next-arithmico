use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Definition {
    pub symbol: String,
    pub expression: Box<Node>,
}

impl Definition {
    pub fn new(symbol: impl Into<String>, expression: Node) -> Node {
        Node::Definition(Definition {
            symbol: symbol.into(),
            expression: expression.into(),
        })
    }
}
