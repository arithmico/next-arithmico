use std::ops::Deref;

use html::Div;
use leptos::*;
use logging::log;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Node;

use crate::{
    command::InsertTextCommand, core::node_mapping::EditorState, node::TextNode,
};

#[component]
pub fn Editor(#[prop(into, optional)] class: Option<String>) -> impl IntoView {
    let editor_ref = create_node_ref::<Div>();
    let (_editor_state, set_editor_state) = create_signal({
        let mut editor_state = EditorState::new();
        editor_state.insert_node(
            Box::new(TextNode::new_with_content("hello ")),
            None,
            None,
        );

        editor_state.insert_node(
            Box::new(TextNode::new_with_content("world ")),
            None,
            None,
        );

        editor_state.insert_node(
            Box::new(TextNode::new_with_content("test")),
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
            _ => (),
        }
    };

    view! { <div class=class ref=editor_ref contenteditable on:beforeinput=beforeinput></div> }
}
