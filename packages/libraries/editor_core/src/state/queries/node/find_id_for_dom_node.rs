use web_sys::Node;

use crate::state::EditorState;

impl EditorState {
    pub fn find_id_for_dom_node(&self, dom_node: &Node) -> Option<usize> {
        self.dom_nodes.iter().find_map(|(node_id, node)| {
            node.as_ref()
                .map(|node| node.eq(dom_node).then(|| *node_id))
                .flatten()
        })
    }
}
