use crate::Node;

pub trait IntoNode {
    fn into_node(self) -> Node;
}
