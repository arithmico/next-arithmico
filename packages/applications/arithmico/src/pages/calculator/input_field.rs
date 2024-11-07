use editor::{
    editor::Editor, editor_provider::EditorProvider,
    use_editor_context::use_editor_context,
};
use leptos::*;

use crate::{state::AppAction, utils::expect_dispatch};

#[component]
pub fn InputField() -> impl IntoView {
    view! {
        <EditorProvider>
            <InputFieldEditor />
        </EditorProvider>
    }
}

#[component]
fn InputFieldEditor() -> impl IntoView {
    let editor_state = use_editor_context();
    let dispatch = expect_dispatch();

    view! {
        <Editor
            on:keydown=move |event| {
                if event.key() == "Enter" {
                    event.prevent_default();
                    editor_state
                        .with_untracked(|state| {
                            if let Some(content) = state.serialize_node(state.get_root_id()) {
                                dispatch.call(AppAction::Evaluate(content))
                            }
                        });
                }
            }
            class="p-2 text-xl whitespace-pre-wrap bg-white rounded-sm border outline-none focus-visible:border-black border-neutral-300"
        />
    }
}
