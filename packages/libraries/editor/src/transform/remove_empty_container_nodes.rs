use editor_core::{
    EditorNode, EditorState, EditorTransform, selection::SelectionRange,
};

pub struct RemoveEmptyContainerNodesTransform;

impl Default for RemoveEmptyContainerNodesTransform {
    fn default() -> Self {
        Self
    }
}

fn find_first_deletable_container(
    state: &EditorState,
) -> Result<Option<usize>, editor_core::Error> {
    let container_nodes = state.find_all_container_nodes();
    for node_id in container_nodes {
        let Ok(EditorNode::Container(node)) = state.get_node(node_id) else {
            continue;
        };

        if !node.delete_if_empty() {
            continue;
        }

        if !state.is_empty_container(node_id)? {
            continue;
        }

        // do not delete the first or last empty container node
        if state.get_previous_leaf_node(node_id)?.is_none()
            && state.get_next_leaf_node(node_id)?.is_none()
        {
            continue;
        }

        return Ok(Some(node_id));
    }
    Ok(None)
}

impl EditorTransform for RemoveEmptyContainerNodesTransform {
    fn transform(
        &self,
        state: &mut EditorState,
    ) -> Result<bool, editor_core::Error> {
        let mut modified = false;

        while let Some(node_id) = find_first_deletable_container(state)? {
            let Some(children_count) = state.get_children_count(node_id)?
            else {
                continue;
            };
            if children_count == 1 {
                let child_id = state.get_child_id_at(node_id, 0)?;
                let child = state.get_leaf_node(child_id)?.arc_clone();
                let length = child.length();
                state.replace_node(node_id, EditorNode::Leaf(child))?;
                state.set_selection(SelectionRange::new_at(node_id, length));
            } else {
                if let Some(previous_leaf_node_id) =
                    state.get_previous_leaf_node(node_id)?
                {
                    let leaf_node_length =
                        state.get_leaf_node(previous_leaf_node_id)?.length();
                    state.delete_node(node_id)?;
                    state.set_selection(SelectionRange::new_at(
                        previous_leaf_node_id,
                        leaf_node_length,
                    ));
                } else if let Some(next_leaf_node_id) =
                    state.get_next_leaf_node(node_id)?
                {
                    state.delete_node(node_id)?;
                    state.set_selection(SelectionRange::new_at(
                        next_leaf_node_id,
                        0,
                    ));
                } else {
                    continue;
                }
            }

            modified = true;
        }
        Ok(modified)
    }
}
