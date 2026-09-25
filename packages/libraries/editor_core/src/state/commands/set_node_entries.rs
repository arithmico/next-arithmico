use send_wrapper::SendWrapper;
use web_sys::Node;

use crate::{EditorNode, state::EditorState};

impl EditorState {
    pub fn set_node_entries(
        &mut self,
        node_id: usize,
        editor_node: EditorNode,
        dom_node: Option<Node>,
        children: Option<Vec<usize>>,
        parent: Option<usize>,
    ) {
        self.editor_nodes.insert(node_id, editor_node);
        self.dom_nodes
            .insert(node_id, dom_node.map(SendWrapper::new));
        self.children.insert(node_id, children);
        self.parent.insert(node_id, parent);
        self.mark_node_id_as_modified(node_id);
    }
}
