use crate::core::Node;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ArgumentMappingEntry {
    Value(Node),
    ValueList(Vec<Node>),
    None,
}
