use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn get_child_dom_node_at(
        &self,
        node_id: usize,
        position: usize,
    ) -> Result<Node, crate::Error> {
        let child_id = self
            .get_child_id_at(node_id, position)
            .ok_or_else(|| crate::Error::NodeNotFound)?;
        self.get_dom_node(child_id)
    }

    pub fn get_children_dom_nodes(&self, node_id: usize) -> Vec<Option<Node>> {
        self.get_children_ids(node_id)
            .into_iter()
            .map(|child_id| self.get_dom_node(child_id).ok())
            .collect::<Vec<_>>()
    }
}
