use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn get_dom_node(&self, node_id: usize) -> Result<Node, crate::Error> {
        self.dom_nodes
            .get(&node_id)
            .cloned()
            .flatten()
            .as_deref()
            .cloned()
            .ok_or_else(|| crate::Error::MissingDomNode)
    }
}
