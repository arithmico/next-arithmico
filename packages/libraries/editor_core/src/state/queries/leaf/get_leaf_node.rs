use std::sync::Arc;

use crate::{EditorLeafNode, EditorNode, EditorState};

impl EditorState {
    pub fn get_leaf_node(
        &self,
        node_id: usize,
    ) -> Result<&Arc<dyn EditorLeafNode>, crate::Error> {
        match self.get_node(node_id)? {
            EditorNode::Container(_) => Err(crate::Error::NotALeafNode),
            EditorNode::Leaf(node) => Ok(node),
        }
    }
}
