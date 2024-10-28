use leptos_dom::log;

use crate::{
    core::{EditorCommand, EditorState, SelectionRange},
    node::TextNode,
};

pub struct InsertTextCommand {
    text: String,
}

impl InsertTextCommand {
    pub fn new<T: ToString>(from: T) -> Self {
        Self {
            text: from.to_string(),
        }
    }

    fn insert_text_into_node_with_children_support(
        &self,
        state: &mut EditorState,
        node_id: usize,
        offset: usize,
    ) {
        if let Some(child_id) =
            state.node_mapping().get_child_at(node_id, offset)
        {
            if let Some(child) =
                state.node_mapping().downcast_node::<TextNode>(child_id)
            {
                let new_child =
                    TextNode::new_with_content(&self.text).append(child);
                state
                    .node_mapping_mut()
                    .replace_node(child_id, new_child.into());
                state
                    .node_mapping_mut()
                    .set_selection(SelectionRange::new_at(
                        child_id,
                        self.text.len(),
                    ));
                return;
            }
        }
        let node_id = state.insert_node(
            TextNode::new_with_content(&self.text).into(),
            Some(node_id),
            Some(offset),
        );
        state
            .node_mapping_mut()
            .set_selection(SelectionRange::new_at(node_id, self.text.len()));
    }
}

impl EditorCommand for InsertTextCommand {
    fn apply(&self, state: &mut EditorState) {
        let Some(range) = state.node_mapping().get_selection() else {
            return;
        };
        let nodes_between = state.node_mapping().get_nodes_between(
            range.get_anchor().get_node_id(),
            range.get_focus().get_node_id(),
        );
        log!("nodes between: {:?}", nodes_between);

        if !range.is_collapsed() {
            return;
        }
        let node_id = range.get_focus().get_node_id();
        let offset = range.get_focus().get_offset();

        if state.node_mapping().supports_children(node_id) {
            return self.insert_text_into_node_with_children_support(
                state, node_id, offset,
            );
        }

        if let Some(text_node) =
            state.node_mapping().downcast_node::<TextNode>(node_id)
        {
            let position = range.get_focus().get_offset();
            let new_node =
                text_node.insert_text(self.text.clone(), position, position);

            state
                .node_mapping_mut()
                .replace_node(node_id, new_node.into());
            state
                .node_mapping_mut()
                .set_selection(SelectionRange::new_at(
                    node_id,
                    position + self.text.len(),
                ));
        }
    }
}
