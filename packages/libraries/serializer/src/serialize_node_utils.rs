use ast::Node;

use crate::{
    error::SerializeNodeError, serialize_node_options::SerializeNodeOptions,
};

pub(crate) trait SerializeNodeUtils {
    fn prepare_serialization(
        &self,
        options: &SerializeNodeOptions,
    ) -> Result<Node, SerializeNodeError>;
}
