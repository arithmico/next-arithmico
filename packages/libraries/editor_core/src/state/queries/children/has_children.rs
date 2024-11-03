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
}
