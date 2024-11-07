use crate::state::EditorState;

impl EditorState {
    pub fn has_children(&self, node_id: usize) -> bool {
        self.children
            .get(&node_id)
            .map(|children| {
                children.as_ref().map(|children| !children.is_empty())
            })
            .flatten()
            .unwrap_or(false)
    }

    pub fn is_empty_container(&self, node_id: usize) -> bool {
        if !self.is_container_node(node_id) {
            return false;
        }
        self.get_children_ids(node_id).is_empty()
    }
}
