use crate::{EditorNode, EditorState};

impl EditorState {
    // TODO: consider Result instead of Option
    pub fn serialize_node(&self, node_id: usize) -> Option<String> {
        let node = self.get_node(node_id).ok()?;
        match node {
            EditorNode::Container(_) => self
                .get_children_ids(node_id)
                .into_iter()
                .map(|child_id| self.serialize_node(child_id))
                .collect::<Option<String>>(),
            EditorNode::Leaf(node) => Some(node.serialize()),
        }
    }
}
