use leptos::{html, prelude::*};

use crate::form::listbox::{
    listbox_button::ListboxButton, listbox_options::ListboxOptions,
};

use super::{listbox_context::ListboxContext, ListboxDefinition};

pub fn use_listbox_is_open() -> Signal<bool> {
    let context = expect_context::<Signal<ListboxIsOpen>>();
    Signal::derive(move || context.get().0)
}

#[derive(Clone)]
struct ListboxIsOpen(bool);

#[component]
pub fn Listbox<V: Send + Sync + Clone + PartialEq + 'static>(
    definition: ListboxDefinition<V>,
    #[prop(into)] on_change: Callback<V>,
    #[prop(into)] value: Signal<V>,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();
    let container_ref = NodeRef::<html::Div>::new();
    let on_change = Callback::new(move |value| {
        on_change.run(value);
        if let Some(button_ref) = button_ref.get_untracked() {
            button_ref.focus().expect("focus");
        }
    });
    let on_cancel = Callback::new(move |()| {
        if let Some(button_ref) = button_ref.get_untracked() {
            button_ref.focus().expect("focus");
        }
    });
    let listbox_context = RwSignal::new(ListboxContext::new(
        &definition,
        &value.get_untracked(),
        on_change,
        on_cancel,
    ));

    provide_context(listbox_context);
    provide_context(Signal::derive(move || {
        ListboxIsOpen(listbox_context.get().is_open())
    }));

    Effect::new({
        let definition = definition.clone();
        move |_| {
            let position = definition.position_of(&value.get());
            if let Some(position) = position {
                listbox_context.update(|listbox_context| {
                    listbox_context.set_position(position)
                });
            }
        }
    });

    view! {
        <div node_ref=container_ref class="listbox-container">
            <ListboxButton
                listbox_definition=definition.clone()
                button_ref=button_ref
            />

            <div class="listbox-options-container">
                <Show when=move || listbox_context.get().is_open()>
                    <ListboxOptions
                        definition=definition.clone()
                        value=value
                        container_ref=container_ref
                    />
                </Show>
            </div>
        </div>
    }
}
