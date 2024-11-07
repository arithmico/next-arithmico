use editor_core::EditorState;
use leptos::{use_context, RwSignal};

pub fn use_editor_context() -> RwSignal<EditorState> {
    use_context::<RwSignal<EditorState>>().expect("editor state")
}
