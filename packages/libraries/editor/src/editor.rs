use std::ops::Deref;

use html::Div;
use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Node;

use crate::{
    command::{
        DeleteContentBackwardCommand, DeleteContentForwardCommand,
        DeleteWordBackwardCommand, DeleteWordForwardCommand, InsertTextCommand,
    },
    use_editor_context::use_editor_context,
};

#[component]
pub fn Editor(
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, into)] data_test_id: Option<String>,
    #[prop(default = false)] autofocus: bool,
) -> impl IntoView {
    let editor_ref = create_node_ref::<Div>();
    let editor_state = use_editor_context();
    let handler = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
        editor_state.update_untracked(move |state| {
            state.read_selection_from_dom();
        });
    });
    document()
        .add_event_listener_with_callback("selectionchange", {
            handler.as_ref().unchecked_ref()
        })
        .expect("add event listener");
    handler.forget();

    create_effect(move |_| {
        let node_ref = editor_ref.get();
        if node_ref.is_none() {
            return;
        }
        let div = node_ref.unwrap();
        if autofocus {
            div.focus().expect("focus");
        }

        let node = div
            .into_any()
            .deref()
            .clone()
            .dyn_into::<Node>()
            .expect("node");

        editor_state.update_untracked(move |state| {
            state.apply_transforms();
            state.mount_to_root(node);
        });
    });

    let beforeinput = move |event: web_sys::InputEvent| {
        event.prevent_default();

        match event.input_type().as_str() {
            "insertText" => {
                let command =
                    InsertTextCommand::new(event.data().expect("data"));
                editor_state.update(|state| {
                    state.execute_command(command.into());
                });
            }
            "deleteContentBackward" => {
                editor_state.update(|state| {
                    state.execute_command(
                        DeleteContentBackwardCommand::new().into(),
                    );
                });
            }
            "deleteContentForward" => {
                editor_state.update(|state| {
                    state.execute_command(
                        DeleteContentForwardCommand::new().into(),
                    );
                });
            }
            "deleteWordBackward" => {
                editor_state.update(|state| {
                    state.execute_command(
                        DeleteWordBackwardCommand::new().into(),
                    );
                });
            }
            "deleteWordForward" => {
                editor_state.update(|state| {
                    state.execute_command(
                        DeleteWordForwardCommand::new().into(),
                    );
                });
            }
            "insertFromPaste" => {
                let data = event
                    .data_transfer()
                    .expect("data transfer")
                    .get_data("text/plain")
                    .expect("data");

                editor_state.update(|state| {
                    state.execute_command(InsertTextCommand::new(data).into());
                });
            }
            _ => (),
        }
    };

    view! {
        <div
            data-testid=data_test_id
            role="textbox"
            class=class
            ref=editor_ref
            contenteditable
            on:beforeinput=beforeinput
        ></div>
    }
}
