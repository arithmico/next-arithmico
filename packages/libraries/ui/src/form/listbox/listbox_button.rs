use leptos::{html, prelude::*};

use crate::form::listbox::listbox_context::ListboxContext;

use super::ListboxDefinition;

#[component]
pub fn ListboxButton<V: Send + Sync + Clone + PartialEq + 'static>(
    listbox_definition: ListboxDefinition<V>,
    button_ref: NodeRef<html::Button>,
) -> impl IntoView {
    let listbox_context = expect_context::<RwSignal<ListboxContext<V>>>();

    view! {
        <button
            id=move || {
                format!(
                    "widget-{}-listbox-button",
                    listbox_context.get().widget_id(),
                )
            }

            type="button"

            aria-haspopup="listbox"

            aria-expanded=move || {
                listbox_context.get().is_open().to_string()
            }

            aria-controls=move || {
                if listbox_context.get().is_open() {
                    Some(
                        format!(
                            "widget-{}-listbox-options",
                            listbox_context.get().widget_id(),
                        ),
                    )
                } else {
                    None
                }
            }

            node_ref=button_ref

            class="listbox-button"

            on:click=move |_| {
                listbox_context
                    .update(|listbox_context| listbox_context.toggle());
            }
        >
            {listbox_definition.button.run()}
        </button>
    }
}
