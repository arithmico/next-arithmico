use web_sys::Node;

use crate::core::EditorNode;

use super::EditorState;

impl EditorState {
    pub fn get_node(&self, node_id: usize) -> Option<&Box<dyn EditorNode>> {
        self.editor_nodes.get(&node_id)
    }

    pub fn get_dom_node(&self, node_id: usize) -> Option<&Node> {
        self.dom_nodes
            .get(&node_id)
            .map(|node| node.as_ref())
            .flatten()
    }

    pub fn get_dom_node_id(&self, dom_node: &Node) -> Option<usize> {
        self.dom_nodes.iter().find_map(|(node_id, node)| {
            node.as_ref()
                .map(|node| node.eq(dom_node).then(|| *node_id))
                .flatten()
        })
    }

    pub fn downcast_node<T: EditorNode>(&self, node_id: usize) -> Option<&T> {
        let node = self.get_node(node_id)?;
        node.as_any().downcast_ref::<T>()
    }
}
