use leptos::{html, prelude::*};

use crate::form::listbox::{
    listbox_context::ListboxContext, listbox_option::ListboxOption,
};

use super::ListboxDefinition;

#[component]
pub fn ListboxOptions<V: Send + Sync + Clone + PartialEq + 'static>(
    definition: ListboxDefinition<V>,
    value: Signal<V>,
    container_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let listbox_context = expect_context::<RwSignal<ListboxContext<V>>>();

    view! {
        <ul
            id=move || {
                format!(
                    "widget-{}-listbox-options",
                    listbox_context.get().widget_id(),
                )
            }

            role="listbox"

            class="listbox-options"

            aria-orientation="vertical"

            aria-labelledby=move || {
                format!(
                    "widget-{}-listbox-button",
                    listbox_context.get().widget_id(),
                )
            }

            aria-activedescendant=move || {
                let listbox_context = listbox_context.get();
                if listbox_context.is_open() {
                    Some(
                        format!(
                            "widget-{}-listbox-option-{}",
                            listbox_context.widget_id(),
                            listbox_context.get_position(),
                        ),
                    )
                } else {
                    None
                }
            }
        >

            {definition
                .options
                .iter()
                .cloned()
                .enumerate()
                .map({
                    let value = value.clone();
                    move |(pos, option)| {
                        view! {
                            <ListboxOption
                                pos=pos
                                option=option
                                value=value
                                container_ref=container_ref
                            />
                        }
                    }
                })
                .collect_view()}
        </ul>
    }
}
