use crate::{state::EditorState, EditorContainerNode, EditorLeafNode};

impl EditorState {
    pub fn get_container_node_as<T: EditorContainerNode>(
        &self,
        node_id: usize,
    ) -> Option<&T> {
        let node = self.get_node(node_id)?;
        assert!(node.supports_children());
        node.as_any().downcast_ref::<T>()
    }

    pub fn get_leaf_node_as<T: EditorLeafNode>(
        &self,
        node_id: usize,
    ) -> Option<&T> {
        let node = self.get_node(node_id)?;
        assert!(!node.supports_children());
        node.as_any().downcast_ref::<T>()
    }
}
