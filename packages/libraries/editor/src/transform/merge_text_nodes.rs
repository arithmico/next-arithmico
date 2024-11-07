use editor_core::{
    selection::{SelectionRange, SelectionRangePoint},
    EditorLeafNode, EditorState, EditorTransform,
};

use crate::node::TextNode;

pub struct MergeTextNodesTransform;

impl MergeTextNodesTransform {
    pub fn new() -> Self {
        Self
    }
}

fn find_first_text_node_child(
    state: &EditorState,
    node_id: usize,
) -> Option<usize> {
    let children = state.get_children_ids(node_id);
    for child_id in children {
        if let Some(_) = state.get_leaf_node_as::<TextNode>(child_id) {
            return Some(child_id);
        }
    }
    None
}

fn find_transform_candidate(
    state: &EditorState,
    node_id: usize,
) -> Option<(usize, usize)> {
    let first_text_node_id = find_first_text_node_child(state, node_id)?;
    let next_sibling_id = state.get_next_sibling_id(first_text_node_id)?;
    if let Some(_) = state.get_leaf_node_as::<TextNode>(next_sibling_id) {
        Some((first_text_node_id, next_sibling_id))
    } else {
        None
    }
}

fn transform_container_node(state: &mut EditorState, node_id: usize) {
    while let Some((first_node_id, second_node_id)) =
        find_transform_candidate(state, node_id)
    {
        let (new_node, first_node_length) = {
            let first_node =
                state.get_leaf_node_as::<TextNode>(first_node_id).unwrap();
            let second_node =
                state.get_leaf_node_as::<TextNode>(second_node_id).unwrap();

            (first_node.append(second_node), first_node.length())
        };

        state.replace_node(first_node_id, new_node.into_editor_node());
        state.delete_node(second_node_id);

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
    }
}

impl EditorTransform for MergeTextNodesTransform {
    fn transform(&self, state: &mut editor_core::EditorState) {
        let container_nodes = state.find_all_container_nodes();
        for node_id in container_nodes {
            transform_container_node(state, node_id);
        }
    }
}
