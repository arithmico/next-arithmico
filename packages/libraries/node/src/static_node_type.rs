use crate::{Node, NodeType};

pub trait GetStaticNodeType {
    fn static_node_type() -> NodeType;
}

impl GetStaticNodeType for Node {
    fn static_node_type() -> NodeType {
        NodeType::Any
    }
}

impl<T: GetStaticNodeType> GetStaticNodeType for Option<T> {
    fn static_node_type() -> NodeType {
        T::static_node_type()
    }
}

impl<T: GetStaticNodeType> GetStaticNodeType for Vec<T> {
    fn static_node_type() -> NodeType {
        T::static_node_type()
    }
}
