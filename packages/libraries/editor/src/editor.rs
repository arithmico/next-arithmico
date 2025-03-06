use std::ops::Deref;

use editor_core::{selection::SelectionRange, EditorLeafNode, EditorState};
use leptos::{html::Div, prelude::*};
use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    Event, Node,
};

use crate::{
    command::{
        DeleteContentBackwardCommand, DeleteContentForwardCommand,
        DeleteWordBackwardCommand, DeleteWordForwardCommand, InsertTextCommand,
    },
    node::TextNode,
};

pub struct EditorStateMutation {
    mutation: Box<dyn Fn(&mut EditorState)>,
}

impl EditorStateMutation {
    pub fn new(f: impl Fn(&mut EditorState) + 'static) -> Self {
        Self {
            mutation: Box::new(f),
        }
    }

    pub fn run(&self, state: &mut EditorState) {
        (self.mutation)(state)
    }
}

struct Listener {
    element: web_sys::EventTarget,
    name: String,
    cb: Closure<dyn Fn(Event)>,
}

impl Listener {
    fn new<F>(element: web_sys::EventTarget, name: impl ToString, cb: F) -> Self
    where
        F: Fn(Event) + 'static,
    {
        let cb = Closure::new(cb);
        let name = name.to_string();

        element
            .add_event_listener_with_callback(
                &name,
                cb.as_ref().unchecked_ref(),
            )
            .unwrap();

        Self { element, name, cb }
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback(
                &self.name,
                self.cb.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}

#[component]
pub fn Editor(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(optional, into)] data_test_id: Option<String>,
    #[prop(default = false)] autofocus: bool,
    update_editor_state: Callback<EditorStateMutation>,
) -> impl IntoView {
    let editor_ref = NodeRef::<Div>::new();

    RwSignal::new_local(Listener::new(
        document().dyn_into().expect("event target"),
        "selectionchange",
        move |_: web_sys::Event| {
            update_editor_state.run(EditorStateMutation::new(|state| {
                state.read_selection_from_dom();
            }));
        },
    ));

    Effect::new(move |_| {
        let node_ref = editor_ref.get();
        if node_ref.is_none() {
            return;
        }
        let div = node_ref.unwrap();
        if autofocus {
            div.focus().expect("focus");
        }

        let node = div.deref().clone().dyn_into::<Node>().expect("node");

        update_editor_state.run(EditorStateMutation::new(move |state| {
            if state.get_selection().is_none() {
                if let Some(node_id) = state.get_all_leaf_node_ids().last() {
                    let length =
                        state.get_leaf_node(*node_id).expect("node").length();
                    state.set_selection(SelectionRange::new_at(
                        *node_id,
                        length.checked_sub(1).unwrap_or(0),
                    ));
                } else {
                    let root_id = state.get_root_id();
                    let node_id = state.insert_node(
                        TextNode::new().into_editor_node(),
                        Some(root_id),
                        None,
                    );
                    state.set_selection(SelectionRange::new_at(node_id, 0));
                }
            }

            state.apply_transforms();
            state.mount_to_root(node.clone());
            state.write_selection_to_dom();
        }));
    });

    let beforeinput = move |event: web_sys::InputEvent| {
        event.prevent_default();

        update_editor_state.run(EditorStateMutation::new(move |state| {
            match event.input_type().as_str() {
                "insertText" => {
                    let command =
                        InsertTextCommand::new(event.data().expect("data"));
                    state.execute_command(command.into());
                }
                "deleteContentBackward" => {
                    state.execute_command(
                        DeleteContentBackwardCommand::new().into(),
                    );
                }
                "deleteContentForward" => {
                    state.execute_command(
                        DeleteContentForwardCommand::new().into(),
                    );
                }
                "deleteWordBackward" => {
                    state.execute_command(
                        DeleteWordBackwardCommand::new().into(),
                    );
                }
                "deleteWordForward" => {
                    state.execute_command(
                        DeleteWordForwardCommand::new().into(),
                    );
                }
                "insertFromPaste" => {
                    let data = event
                        .data_transfer()
                        .expect("data transfer")
                        .get_data("text/plain")
                        .expect("data");

                    state.execute_command(InsertTextCommand::new(data).into());
                }
                _ => (),
            }
        }));
    };

    view! {
        <div
            id=id
            data-testid=data_test_id
            role="textbox"
            class=class
            node_ref=editor_ref
            contenteditable
            on:beforeinput=beforeinput
        ></div>
    }
}
