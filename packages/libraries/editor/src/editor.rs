use std::ops::Deref;

use editor_core::{EditorLeafNode, EditorState};
use html::Div;
use leptos::*;
use logging::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Node;

use crate::{
    command::{
        DeleteContentBackwardCommand, DeleteContentForwardCommand,
        DeleteWordBackwardCommand, DeleteWordForwardCommand, InsertTextCommand,
    },
    node::TextNode,
    transform::TextTransform,
};

#[component]
pub fn Editor(#[prop(into, optional)] class: Option<String>) -> impl IntoView {
    let editor_ref = create_node_ref::<Div>();
    let (_editor_state, set_editor_state) = create_signal({
        let mut editor_state = EditorState::new();
        editor_state.add_transform(TextTransform);
        editor_state.insert_node(
            TextNode::new_with_content("hello ").into_editor_node(),
            None,
            None,
        );

        editor_state.insert_node(
            TextNode::new_with_content("world ").into_editor_node(),
            None,
            None,
        );

        editor_state.insert_node(
            TextNode::new_with_content("test").into_editor_node(),
            None,
            None,
        );
        editor_state
    });
    let handler = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
        set_editor_state.update_untracked(move |state| {
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
        let node = node_ref
            .unwrap()
            .into_any()
            .deref()
            .clone()
            .dyn_into::<Node>()
            .expect("node");

        set_editor_state.update_untracked(move |state| {
            //state.apply_transforms();
            state.mount_to_root(node);
        });
    });

    let beforeinput = move |event: web_sys::InputEvent| {
        event.prevent_default();

        log!("input_type: {}", event.input_type());
        match event.input_type().as_str() {
            "insertText" => {
                let command =
                    InsertTextCommand::new(event.data().expect("data"));
                set_editor_state.update(|state| {
                    state.execute_command(command.into());
                });
            }
            "deleteContentBackward" => {
                set_editor_state.update(|state| {
                    state.execute_command(
                        DeleteContentBackwardCommand::new().into(),
                    );
                });
            }
            "deleteContentForward" => {
                set_editor_state.update(|state| {
                    state.execute_command(
                        DeleteContentForwardCommand::new().into(),
                    );
                });
            }
            "deleteWordBackward" => {
                set_editor_state.update(|state| {
                    state.execute_command(
                        DeleteWordBackwardCommand::new().into(),
                    );
                });
            }
            "deleteWordForward" => {
                set_editor_state.update(|state| {
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

                set_editor_state.update(|state| {
                    state.execute_command(InsertTextCommand::new(data).into());
                });
            }
            _ => (),
        }
    };

    view! { <div class=class ref=editor_ref contenteditable on:beforeinput=beforeinput></div> }
}
