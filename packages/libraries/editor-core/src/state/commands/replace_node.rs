use crate::{EditorNode, state::EditorState};

impl EditorState {
    pub fn replace_node(
        &mut self,
        node_id: usize,
        node: EditorNode,
    ) -> Result<(), crate::Error> {
        let is_container = node.supports_children();
        self.editor_nodes.insert(node_id, node);
        self.dom_nodes.insert(node_id, None);
        let previous_children = self.children.insert(
            node_id,
            if is_container { Some(Vec::new()) } else { None },
        );
        if let Some(children) = previous_children.flatten() {
            self.delete_many_nodes(children.into_iter().collect())?;
        }
        self.modified_nodes.insert(node_id);
        Ok(())
    }
}
