use std::ops::Deref;

use editor_core::{
    EditorLeafNode, EditorState,
    selection::{AbsoluteSelectionRange, SelectionRange},
};
use leptos::{html::Div, prelude::*};
use leptos_dom::error;
use web_sys::{
    Event, EventTarget, Node,
    wasm_bindgen::{JsCast, JsValue, prelude::Closure},
};

use crate::{
    command::{
        DeleteByCutCommand, DeleteContentBackwardCommand,
        DeleteContentForwardCommand, DeleteWordBackwardCommand,
        DeleteWordForwardCommand, InsertTextCommand,
    },
    node::TextNode,
};

pub struct EditorStateMutation {
    #[allow(clippy::type_complexity)]
    mutation: Box<dyn Fn(&mut EditorState) -> Result<(), editor_core::Error>>,
}

impl EditorStateMutation {
    pub fn new(
        f: impl Fn(&mut EditorState) -> Result<(), editor_core::Error> + 'static,
    ) -> Self {
        Self {
            mutation: Box::new(f),
        }
    }

    pub fn run(
        &self,
        state: &mut EditorState,
    ) -> Result<(), editor_core::Error> {
        (self.mutation)(state)
    }
}

struct Listener {
    element: web_sys::EventTarget,
    name: String,
    cb: Closure<dyn Fn(Event)>,
}

impl Listener {
    fn new<F>(
        element: web_sys::EventTarget,
        name: impl ToString,
        cb: F,
    ) -> Result<Self, JsValue>
    where
        F: Fn(Event) + 'static,
    {
        let cb = Closure::new(cb);
        let name = name.to_string();

        element.add_event_listener_with_callback(
            &name,
            cb.as_ref().unchecked_ref(),
        )?;

        Ok(Self { element, name, cb })
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        if let Err(err) = self.element.remove_event_listener_with_callback(
            &self.name,
            self.cb.as_ref().unchecked_ref(),
        ) {
            error!("Failed to remove event listener: {:?}", err);
        }
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
    let composition_trigger = RwSignal::<Option<String>>::new(None);
    let selection_before_composition =
        RwSignal::<Option<AbsoluteSelectionRange>>::new(None);

    let document_element = match document().dyn_into::<EventTarget>() {
        Ok(element) => element,
        Err(err) => {
            error!("Failed to cast Document into EventTarget: {:?}", err);
            return ().into_any();
        }
    };

    RwSignal::new_local(Listener::new(
        document_element,
        "selectionchange",
        move |_: web_sys::Event| {
            update_editor_state.run(EditorStateMutation::new(move |state| {
                state.read_selection_from_dom();
                Ok(())
            }));
        },
    ));

    Effect::new(move |_| {
        let node_ref = editor_ref.get();
        let Some(div) = node_ref else {
            return;
        };

        if autofocus && let Err(err) = div.focus() {
            error!("Failed to focus element: {:?}", err);
        }

        let node = match div.deref().clone().dyn_into::<Node>() {
            Ok(node) => node,
            Err(err) => {
                error!("Failed to cast Div into Node: {:?}", err);
                return;
            }
        };

        update_editor_state.run(EditorStateMutation::new(move |state| {
            if state.get_selection().is_none() {
                if let Some(node_id) = state.get_all_leaf_node_ids().last() {
                    let length = state.get_leaf_node(*node_id)?.length();
                    state.set_selection(SelectionRange::new_at(
                        *node_id,
                        length.saturating_sub(1),
                    ));
                } else {
                    let root_id = state.get_root_id();
                    let node_id = state.insert_node(
                        TextNode::default().into_editor_node(),
                        Some(root_id),
                        None,
                    )?;
                    state.set_selection(SelectionRange::new_at(node_id, 0));
                }
            }

            state.apply_transforms()?;
            state.mount_to_root(node.clone())?;
            state.write_selection_to_dom()?;
            Ok(())
        }));
    });

    let beforeinput = move |event: web_sys::InputEvent| {
        event.prevent_default();

        update_editor_state.run(EditorStateMutation::new(move |state| {
            match event.input_type().as_str() {
                "insertText" => {
                    let command = InsertTextCommand::new(
                        event.data().unwrap_or_default(),
                    );
                    state.execute_command(command.into())
                }
                "deleteContentBackward" => {
                    state.execute_command(DeleteContentBackwardCommand.into())
                }
                "deleteContentForward" => {
                    state.execute_command(DeleteContentForwardCommand.into())
                }
                "deleteWordBackward" => {
                    state.execute_command(DeleteWordBackwardCommand.into())
                }
                "deleteWordForward" => {
                    state.execute_command(DeleteWordForwardCommand.into())
                }
                "insertFromPaste" => {
                    let Some(data_transfer) = event.data_transfer() else {
                        error!("InputEvent does not contain DataTransfer.");
                        return Ok(());
                    };
                    let data = match data_transfer.get_data("text/plain") {
                        Ok(data) => data,
                        Err(err) => {
                            error!("Failed to read text/plain from DataTransfer: {:?}", err);
                            return Ok(())
                        },
                    };

                    state.execute_command(InsertTextCommand::new(data).into())
                }
                "deleteByCut" => {
                    state.execute_command(DeleteByCutCommand.into())
                }
                _ => Ok(()),
            }
        }));
    };

    Effect::new(move || {
        update_editor_state.run(EditorStateMutation::new(move |state| {
            let data = composition_trigger.get();
            if let Some(data) = data {
                if let Some(selection) =
                    selection_before_composition.get_untracked()
                {
                    state.set_absolute_selection(selection);
                }
                let command = InsertTextCommand::new(data);
                state.execute_command(command.into())?;
                selection_before_composition.set(None);
            }
            Ok(())
        }));
    });

    view! {
        <div
            id=id
            data-testid=data_test_id
            role="textbox"
            class=class
            node_ref=editor_ref
            contenteditable
            on:beforeinput=beforeinput
            on:compositionstart=move |_| {
                update_editor_state
                    .run(
                        EditorStateMutation::new(move |state| {
                            selection_before_composition
                                .set(Some(state.get_absolute_selection()?));
                            Ok(())
                        }),
                    );
            }
            on:compositionend=move |event| {
                let data = event.data();
                composition_trigger.set(data);
            }
        ></div>
    }
    .into_any()
}
