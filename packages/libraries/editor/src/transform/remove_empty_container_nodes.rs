use editor_core::{
    EditorNode, EditorState, EditorTransform, selection::SelectionRange,
};

pub struct RemoveEmptyContainerNodesTransform;

impl Default for RemoveEmptyContainerNodesTransform {
    fn default() -> Self {
        Self
    }
}

fn find_first_deletable_container(state: &EditorState) -> Option<usize> {
    let container_nodes = state.find_all_container_nodes();
    let node_id = container_nodes.into_iter().find(|node_id| {
        let Some(EditorNode::Container(node)) = state.get_node(*node_id) else {
            return false;
        };
        if !node.delete_if_empty() {
            return false;
        }
        if state.is_empty_container(*node_id) {
            if state.get_previous_leaf_node(*node_id).is_none()
                && state.get_next_leaf_node(*node_id).is_none()
            {
                return false;
            }
            return true;
        } else if state.get_children_count(*node_id) == Some(1) {
            let child_id =
                state.get_child_id_at(*node_id, 0).expect("child id");
            if let Some(child) = state.get_leaf_node(child_id)
                && child.length() == 0
            {
                return true;
            }
        }
        false
    })?;

    Some(node_id)
}

impl EditorTransform for RemoveEmptyContainerNodesTransform {
    fn transform(&self, state: &mut EditorState) -> bool {
        let mut modified = false;

        while let Some(node_id) = find_first_deletable_container(state) {
            let children_count =
                state.get_children_count(node_id).expect("children count");
            if children_count == 1 {
                let child_id =
                    state.get_child_id_at(node_id, 0).expect("child id");
                let child =
                    state.get_leaf_node(child_id).expect("child").arc_clone();
                let length = child.length();
                state.replace_node(node_id, EditorNode::Leaf(child));
                state.set_selection(SelectionRange::new_at(node_id, length));
            } else {
                if let Some(previous_leaf_node_id) =
                    state.get_previous_leaf_node(node_id)
                {
                    let leaf_node_length = state
                        .get_leaf_node(previous_leaf_node_id)
                        .expect("leaf node")
                        .length();
                    state.delete_node(node_id);
                    state.set_selection(SelectionRange::new_at(
                        previous_leaf_node_id,
                        leaf_node_length,
                    ));
                } else {
                    let next_leaf_node_id = state
                        .get_next_leaf_node(node_id)
                        .expect("next leaf node");
                    state.delete_node(node_id);
                    state.set_selection(SelectionRange::new_at(
                        next_leaf_node_id,
                        0,
                    ));
                }
            }

            modified = true;
        }
        modified
    }
}
