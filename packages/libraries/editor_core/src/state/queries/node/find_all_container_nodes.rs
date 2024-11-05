use std::collections::HashSet;

use crate::EditorState;

impl EditorState {
    pub fn find_all_container_nodes(&self) -> HashSet<usize> {
        let mut container_nodes = HashSet::<usize>::new();
        for node_id in self.editor_nodes.keys().copied() {
            if self.is_container_node(node_id) {
                container_nodes.insert(node_id);
            }
        }
        container_nodes
    }
}
