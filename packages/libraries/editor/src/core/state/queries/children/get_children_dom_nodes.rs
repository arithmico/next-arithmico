use web_sys::Node;

use crate::core::state::EditorState;

impl EditorState {
    pub fn get_children_dom_nodes(&self, node_id: usize) -> Option<Vec<Node>> {
        self.get_children_ids(node_id)
            .into_iter()
            .map(|child_id| self.get_dom_node(child_id).cloned())
            .collect::<Option<Vec<_>>>()
    }
}
