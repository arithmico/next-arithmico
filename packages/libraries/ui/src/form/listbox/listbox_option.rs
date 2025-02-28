use leptos::{html, prelude::*};
use web_sys::{wasm_bindgen::JsCast, Node};

use crate::form::listbox::listbox_context::ListboxContext;

use super::listbox_definition::ListboxOptionDefinition;

#[component]
pub fn ListboxOption<V: PartialEq + Send + Sync + Clone + 'static>(
    pos: usize,
    option: ListboxOptionDefinition<V>,
    value: Signal<V>,
    container_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let listbox_context = expect_context::<RwSignal<ListboxContext<V>>>();

    let node_ref: NodeRef<html::Li> = NodeRef::new();
    let selected = Signal::derive({
        let option_value = option.value.clone();
        move || value.get() == option_value
    });
    Effect::new(move |_| {
        let listbox_context = listbox_context.get();
        if pos == listbox_context.get_position() {
            if let Some(node_ref) = node_ref.get() {
                node_ref.focus().expect("focus");
            }
        }
    });
    view! {
        <li
            id=move || {
                format!(
                    "widget-{}-listbox-option-{}",
                    listbox_context.get().widget_id(),
                    pos,
                )
            }

            role="option"

            aria-selected=move || selected.get().to_string()

            node_ref=node_ref

            tabindex="-1"

            class=move || {
                format!(
                    "listbox-option {}",
                    if selected.get() { "selected" } else { "" },
                )
            }

            on:focusout=move |e| {
                let target = e.related_target();
                if let Some(container_ref) = container_ref.get_untracked() {
                    if let Some(target) = target {
                        let target = target.dyn_into::<Node>().ok();
                        if !container_ref.contains(target.as_ref()) {
                            listbox_context
                                .update(|listbox_context| listbox_context.cancel());
                        }
                    } else {
                        listbox_context
                            .update(|listbox_context| listbox_context.cancel());
                    }
                }
            }

            on:click={
                let value = option.value.clone();
                move |_| {
                    listbox_context
                        .update(|listbox_context| listbox_context.select(&value))
                }
            }

            on:keydown={
                let value = option.value.clone();
                move |event| {
                    listbox_context
                        .update(|listbox_context| {
                            listbox_context.on_keydown(event, &value)
                        })
                }
            }
        >
            {option.view.run()}
        </li>
    }
}
