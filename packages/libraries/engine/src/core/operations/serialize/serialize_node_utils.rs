use crate::core::{Context, Node, SerializeNodeError};

pub trait SerializeNodeUtils {
    fn prepare_serialization(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError>;
}
