use leptos::{html, prelude::*};
use web_sys::{wasm_bindgen::JsCast, Node};

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
                {definition.button.run()}
            </button>
            <div class="listbox-options-container">
                <Show when=move || listbox_context.get().is_open()>
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
                                    let node_ref: NodeRef<html::Li> = NodeRef::new();
                                    let selected = Signal::derive({
                                        let option_value = option.value.clone();
                                        move || { value.get() == option_value }
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
                            })
                            .collect_view()}
                    </ul>
                </Show>
            </div>
        </div>
    }
}
