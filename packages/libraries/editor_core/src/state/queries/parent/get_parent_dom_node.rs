use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn get_parent_dom_node(&self, node_id: usize) -> Option<&Node> {
        self.get_parent_id(node_id)
            .map(|parent_id| self.get_dom_node(parent_id))
            .flatten()
    }
}
