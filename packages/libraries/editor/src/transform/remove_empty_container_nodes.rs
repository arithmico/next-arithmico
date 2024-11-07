use editor_core::{
    selection::SelectionRange, EditorNode, EditorState, EditorTransform,
};

pub struct RemoveEmptyContainerNodesTransform;

impl RemoveEmptyContainerNodesTransform {
    pub fn new() -> Self {
        Self
    }
}

fn find_first_deletable_container(state: &EditorState) -> Option<usize> {
    let container_nodes = state.find_all_container_nodes();
    let node_id = container_nodes.into_iter().find(|node_id| {
        if state.is_empty_container(*node_id) {
            if let Some(node) = state.get_node(*node_id) {
                if let EditorNode::Container(node) = node {
                    return node.delete_if_empty();
                }
            }
        } else if state.get_children_count(*node_id) == Some(1) {
            let child_id =
                state.get_child_id_at(*node_id, 0).expect("child id");
            if let Some(child) = state.get_leaf_node(child_id) {
                if child.length() == 0 {
                    return true;
                }
            }
        }
        false
    })?;

    state.get_previous_leaf_node(node_id)?;

    Some(node_id)
}

impl EditorTransform for RemoveEmptyContainerNodesTransform {
    fn transform(&self, state: &mut EditorState) -> bool {
        let mut modified = false;
        while let Some(node_id) = find_first_deletable_container(state) {
            let previous_leaf_node_id = state
                .get_previous_leaf_node(node_id)
                .expect("previous leaf node id");
            let leaf_node_length = state
                .get_leaf_node(previous_leaf_node_id)
                .expect("leaf node")
                .length();
            state.delete_node(node_id);
            state.set_selection(SelectionRange::new_at(
                previous_leaf_node_id,
                leaf_node_length,
            ));
            modified = true;
        }
        modified
    }
}
