use editor_core::EditorState;
use leptos::prelude::{use_context, LocalStorage, RwSignal};

pub fn use_editor_context() -> RwSignal<EditorState, LocalStorage> {
    use_context::<RwSignal<EditorState, LocalStorage>>().expect("editor state")
}
