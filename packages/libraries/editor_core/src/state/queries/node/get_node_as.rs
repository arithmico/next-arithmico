use crate::{EditorContainerNode, EditorLeafNode, state::EditorState};

impl EditorState {
    pub fn get_container_node_as<T: EditorContainerNode>(
        &self,
        node_id: usize,
    ) -> Result<&T, crate::Error> {
        let node = self.get_node(node_id)?;
        if !node.supports_children() {
            return Err(crate::Error::NotAContainerNode);
        }
        node.as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| crate::Error::NodeCastFailed)
    }

    pub fn get_leaf_node_as<T: EditorLeafNode>(
        &self,
        node_id: usize,
    ) -> Result<&T, crate::Error> {
        let node = self.get_node(node_id)?;
        if node.supports_children() {
            return Err(crate::Error::NotALeafNode);
        }
        node.as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| crate::Error::NodeCastFailed)
    }
}
