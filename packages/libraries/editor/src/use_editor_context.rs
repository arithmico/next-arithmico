use editor_core::EditorState;
use leptos::prelude::{RwSignal, use_context};

pub fn use_editor_context() -> Option<RwSignal<EditorState>> {
    use_context::<RwSignal<EditorState>>()
}
