use leptos::{html::Button, prelude::*};

use super::ListboxButtonDefinition;

#[component]
pub(super) fn InternalButton(
    node_ref: NodeRef<Button>,
    definition: ListboxButtonDefinition,
    is_open: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=definition.class.clone()
            aria-haspopup="listbox"
            aria-expanded=move || is_open.get().to_string()
        >

            {definition.children.clone()}
        </button>
    }
}
