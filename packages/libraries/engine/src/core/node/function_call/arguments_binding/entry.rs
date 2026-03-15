use crate::core::Node;

#[derive(Debug, Clone)]
pub enum ArgumentBindingEntry {
    Value(Node),
    ValueList(Vec<Node>),
    None,
}
