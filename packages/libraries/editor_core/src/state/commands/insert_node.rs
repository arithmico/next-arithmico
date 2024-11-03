use crate::{
    {state::EditorState, EditorNode},
    utils::get_node_id,
};

impl EditorState {
    pub fn insert_node(
        &mut self,
        node: Box<dyn EditorNode>,
        parent_id: Option<usize>,
        position: Option<usize>,
    ) -> usize {
        let node_id = get_node_id();
        let parent_id = parent_id.unwrap_or(self.root_node_id);
        let parent_children = self
            .children
            .get_mut(&parent_id)
            .expect("parent")
            .as_mut()
            .expect("parent supports children");
        let position = position.unwrap_or(parent_children.len());
        parent_children.insert(position, node_id);
        let children = node.supports_children().then(|| vec![]);
        self.set_node_entries(node_id, node, None, children, Some(parent_id));
        node_id
    }
}
