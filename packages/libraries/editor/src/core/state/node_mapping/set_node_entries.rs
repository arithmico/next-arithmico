use web_sys::Node;

use crate::core::EditorNode;

use super::NodeMapping;

impl NodeMapping {
    pub(super) fn set_node_entries(
        &mut self,
        node_id: usize,
        editor_node: Box<dyn EditorNode>,
        dom_node: Option<Node>,
        children: Option<Vec<usize>>,
        parent: Option<usize>,
    ) {
        self.editor_nodes.insert(node_id, editor_node);
        self.dom_nodes.insert(node_id, dom_node);
        self.children.insert(node_id, children);
        self.parent.insert(node_id, parent);
        self.mark_as_modified(node_id);
    }
}
