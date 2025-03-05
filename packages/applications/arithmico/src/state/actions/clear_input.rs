use editor::node::TextNode;
use editor_core::{EditorCommand, EditorLeafNode};
use web_state::WebStateAction;

use crate::state::State;

#[derive(Clone)]
pub struct ClearInputAction;

impl ClearInputAction {
    pub fn new() -> Self {
        Self
    }
}

impl EditorCommand for ClearInputAction {
    fn apply(&self, state: &mut editor_core::EditorState) -> Option<()> {
        let root_children = state.get_children_ids(state.get_root_id());
        if let Some(first) = root_children.first().copied() {
            state.insert_node_before(TextNode::new().into_editor_node(), first);
            state.delete_many_nodes(root_children.into_iter().collect());
        }
        Some(())
    }
}

impl WebStateAction<State> for ClearInputAction {
    fn apply(&self, state: &mut State) {
        state
            .input_editor_state
            .execute_command(Box::new(self.clone()));
    }
}
