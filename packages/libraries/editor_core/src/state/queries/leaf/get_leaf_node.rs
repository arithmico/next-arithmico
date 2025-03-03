use std::sync::Arc;

use crate::{EditorLeafNode, EditorNode, EditorState};

impl EditorState {
    pub fn get_leaf_node(
        &self,
        node_id: usize,
    ) -> Option<&Arc<dyn EditorLeafNode>> {
        match self.get_node(node_id)? {
            EditorNode::Container(_) => None,
            EditorNode::Leaf(node) => Some(node),
        }
    }
}
