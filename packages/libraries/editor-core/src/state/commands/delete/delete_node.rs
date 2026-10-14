use crate::state::EditorState;

impl EditorState {
    /// This does not delete the root node
    pub fn delete_node(&mut self, node_id: usize) -> Result<(), crate::Error> {
        let Some(parent_id) = self.get_parent_id(node_id)? else {
            return Ok(());
        };
        let mut nodes_to_remove = self.get_all_children_ids(node_id);
        nodes_to_remove.insert(node_id);
        for child_id in nodes_to_remove {
            self.delete_node_entries(child_id);
        }
        let new_children = self
            .get_children_ids(parent_id)
            .into_iter()
            .filter(|child_id| *child_id != node_id)
            .collect::<Vec<_>>();
        self.children.insert(parent_id, Some(new_children));
        self.modified_nodes.insert(parent_id);
        Ok(())
    }
}
