use editor_core::{
    EditorLeafNode, EditorState, EditorTransform,
    selection::{SelectionRange, SelectionRangePoint},
};

use crate::node::TextNode;

pub struct MergeTextNodesTransform;

impl Default for MergeTextNodesTransform {
    fn default() -> Self {
        Self
    }
}

fn find_first_text_node_child(
    state: &EditorState,
    node_id: usize,
) -> Option<usize> {
    let children = state.get_children_ids(node_id);

    children
        .into_iter()
        .find(|&child_id| state.get_leaf_node_as::<TextNode>(child_id).is_ok())
}

fn find_transform_candidate(
    state: &EditorState,
    node_id: usize,
) -> Result<Option<(usize, usize)>, editor_core::Error> {
    let Some(first_text_node_id) = find_first_text_node_child(state, node_id)
    else {
        return Ok(None);
    };
    let Some(next_sibling_id) =
        state.get_next_sibling_id(first_text_node_id)?
    else {
        return Ok(None);
    };
    Ok(
        if state.get_leaf_node_as::<TextNode>(next_sibling_id).is_ok() {
            Some((first_text_node_id, next_sibling_id))
        } else {
            None
        },
    )
}

fn transform_container_node(
    state: &mut EditorState,
    node_id: usize,
) -> Result<bool, editor_core::Error> {
    let mut modified = false;
    while let Some((first_node_id, second_node_id)) =
        find_transform_candidate(state, node_id)?
    {
        let (new_node, first_node_length) = {
            let first_node =
                state.get_leaf_node_as::<TextNode>(first_node_id)?;
            let second_node =
                state.get_leaf_node_as::<TextNode>(second_node_id)?;

            (first_node.append(second_node), first_node.length())
        };

        state.replace_node(first_node_id, new_node.into_editor_node())?;
        state.delete_node(second_node_id)?;

        if let Some(selection) = state.get_selection() {
            let anchor =
                if selection.get_anchor().get_node_id() == second_node_id {
                    SelectionRangePoint::new(
                        first_node_id,
                        selection.get_anchor().get_offset() + first_node_length,
                    )
                } else {
                    selection.get_anchor().clone()
                };

            let focus = if selection.get_focus().get_node_id() == second_node_id
            {
                SelectionRangePoint::new(
                    first_node_id,
                    selection.get_focus().get_offset() + first_node_length,
                )
            } else {
                selection.get_focus().clone()
            };

            state.set_selection(SelectionRange::new(anchor, focus));
        }

        modified = true;
    }
    Ok(modified)
}

impl EditorTransform for MergeTextNodesTransform {
    fn transform(
        &self,
        state: &mut editor_core::EditorState,
    ) -> Result<bool, editor_core::Error> {
        let mut modified = false;
        let container_nodes = state.find_all_container_nodes();
        for node_id in container_nodes {
            modified = modified || transform_container_node(state, node_id)?;
        }
        Ok(modified)
    }
}
