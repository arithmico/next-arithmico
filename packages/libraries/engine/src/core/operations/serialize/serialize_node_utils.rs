use crate::core::{Context, Node, SerializeNodeError};

pub trait NormalizeNode {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError>;
}
