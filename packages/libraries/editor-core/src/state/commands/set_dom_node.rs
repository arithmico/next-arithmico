use send_wrapper::SendWrapper;
use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn set_dom_node(&mut self, node_id: usize, dom_node: Node) {
        let previous_dom_node = self
            .dom_nodes
            .insert(node_id, Some(SendWrapper::new(dom_node)));
        assert!(previous_dom_node.is_some());
    }
}
