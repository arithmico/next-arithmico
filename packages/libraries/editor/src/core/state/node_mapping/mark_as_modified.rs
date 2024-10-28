use super::NodeMapping;

impl NodeMapping {
    pub fn mark_as_modified(&mut self, node_id: usize) {
        let mut current_node_id = Some(node_id);
        while let Some(node_id) = current_node_id {
            self.modified_nodes.insert(node_id);
            current_node_id = self.parent_id(node_id);
        }
    }
}
