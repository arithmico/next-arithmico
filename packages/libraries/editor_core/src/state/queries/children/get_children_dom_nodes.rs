use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn get_child_dom_node_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> Option<Node> {
        let child_id = self.get_child_id_at(node_id, position)?;
        self.get_dom_node(child_id)
    }

    pub fn get_children_dom_nodes(&self, node_id: usize) -> Vec<Option<Node>> {
        self.get_children_ids(node_id)
            .into_iter()
            .map(|child_id| self.get_dom_node(child_id))
            .collect::<Vec<_>>()
    }
}
