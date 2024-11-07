use editor_core::{EditorContainerNode, EditorLeafNode, EditorState};
use leptos::*;

use crate::{
    node::{MarkNode, TextNode},
    transform::{MergeTextNodesTransform, RemoveEmptyContainerNodesTransform},
};

#[component]
pub fn EditorProvider(children: Children) -> impl IntoView {
    let editor_state = create_rw_signal({
        let mut editor_state = EditorState::new();
        editor_state.add_transform(MergeTextNodesTransform::new());
        editor_state.add_transform(RemoveEmptyContainerNodesTransform::new());

        editor_state.insert_node(
            TextNode::new_with_content("hello ").into_editor_node(),
            None,
            None,
        );

        let mark_node_id = editor_state.insert_node(
            MarkNode::new().into_editor_node(),
            None,
            None,
        );

        editor_state.insert_node(
            TextNode::new_with_content("world ").into_editor_node(),
            Some(mark_node_id),
            None,
        );

        editor_state.insert_node(
            TextNode::new_with_content("test").into_editor_node(),
            None,
            None,
        );
        editor_state
    });

    provide_context(editor_state);

    view! { <>{children()}</> }
}
