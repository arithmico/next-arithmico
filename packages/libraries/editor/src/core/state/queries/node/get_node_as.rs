use crate::core::{state::EditorState, EditorNode};

impl EditorState {
    pub fn get_node_as<T: EditorNode>(&self, node_id: usize) -> Option<&T> {
        let node = self.get_node(node_id)?;
        node.as_any().downcast_ref::<T>()
    }
}
