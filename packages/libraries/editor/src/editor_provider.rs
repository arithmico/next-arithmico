use editor_core::EditorState;
use leptos::prelude::*;

use crate::transform::{
    MergeTextNodesTransform, RemoveEmptyContainerNodesTransform,
};

#[component]
pub fn EditorProvider(children: Children) -> impl IntoView {
    let editor_state = RwSignal::new_local({
        let mut editor_state = EditorState::new();
        editor_state.add_transform(MergeTextNodesTransform::new());
        editor_state.add_transform(RemoveEmptyContainerNodesTransform::new());
        editor_state
    });

    provide_context(editor_state);

    view! { <>{children()}</> }
}
