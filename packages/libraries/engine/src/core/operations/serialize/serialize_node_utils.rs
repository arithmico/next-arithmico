use node::Node;

use crate::{
    Serialize,
    core::{Context, SerializeNodeError},
};

pub trait SerializeUtils {
    fn normalize_node(
        &self,
        context: &Context,
    ) -> Result<Node, SerializeNodeError>;

    fn child_requires_parenthesis(&self, child: &Node, position: usize)
    -> bool;

    fn serialize_child(
        &self,
        child: &Node,
        position: usize,
        context: &Context,
    ) -> Result<String, SerializeNodeError> {
        let serialized_node = child.serialize(context)?;
        if self.child_requires_parenthesis(child, position) {
            Ok(format!("({})", serialized_node))
        } else {
            Ok(serialized_node)
        }
    }
}
