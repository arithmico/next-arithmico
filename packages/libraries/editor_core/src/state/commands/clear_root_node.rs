use crate::EditorState;

impl EditorState {
    pub fn clear_root_node(&mut self) -> Result<(), crate::Error> {
        let root_children_ids = self.get_children_ids(self.get_root_id());
        self.delete_many_nodes(root_children_ids.into_iter().collect())
    }
}
