use editor_core::EditorState;
use leptos::prelude::{RwSignal, use_context};

pub fn use_editor_context() -> RwSignal<EditorState> {
    use_context::<RwSignal<EditorState>>().expect("editor state")
}
