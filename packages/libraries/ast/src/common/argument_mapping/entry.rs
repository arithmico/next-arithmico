use crate::Node;

#[derive(Debug, Clone)]
pub enum ArgumentMappingEntry {
    Value(Node),
    ValueList(Vec<Node>),
    None,
}
