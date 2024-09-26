use super::Node;

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: &str) -> Node {
        Node::Symbol(Self {
            name: name.to_string(),
        })
    }
}
