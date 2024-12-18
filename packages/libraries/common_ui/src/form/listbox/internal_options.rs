use leptos::prelude::*;

use crate::form::listbox::{internal_option::InternalOption, ListboxContext};

use super::ListboxOptionDefinition;

#[component]
pub(super) fn InternalOptions<T>(
    class: Option<String>,
    options: Vec<ListboxOptionDefinition<T>>,
) -> impl IntoView
where
    T: PartialEq + Clone + 'static,
{
    let context = use_context::<ReadSignal<ListboxContext<T>>>()
        .expect("listbox context");

    view! {
        <Show when=move || { context.get().is_open }>
            <div>
                <ul
                    role="listbox"
                    aria-orientation="vertical"
                    class=class.clone()
                    id=move || format!("listbox-{}-options", context.get().id)
                >

                    {options
                        .iter()
                        .cloned()
                        .enumerate()
                        .map(|(_index, option)| {
                            view! { <InternalOption option=option /> }
                        })
                        .collect_view()}
                </ul>
            </div>
        </Show>
    }
}
