use leptos::{html, prelude::*};
use web_sys::{wasm_bindgen::JsCast, Node};

use super::ListboxDefinition;

use crate::widget_id::use_widget_id;

#[derive(Clone)]
struct ListboxContext {
    is_open: ReadSignal<bool>,
}

pub fn use_listbox_is_open() -> ReadSignal<bool> {
    let context = expect_context::<ListboxContext>();
    context.is_open
}

#[component]
pub fn Listbox<V: Send + Sync + Clone + PartialEq + 'static>(
    definition: ListboxDefinition<V>,
    #[prop(into)] on_change: Callback<V>,
    #[prop(into)] value: Signal<V>,
) -> impl IntoView {
    let widget_id = use_widget_id();
    let is_open = RwSignal::new(false);
    let selected_pos = RwSignal::new(None);
    let length = definition.options.options.len();
    let button_ref = NodeRef::<html::Button>::new();
    let container_ref = NodeRef::<html::Div>::new();

    provide_context(ListboxContext {
        is_open: is_open.split().0,
    });

    Effect::new({
        let definition = definition.clone();
        move |_| {
            let value = value.get();
            if is_open.get() {
                let pos = definition
                    .options
                    .options
                    .iter()
                    .position(|v| v.value.eq(&value));
                selected_pos.set(pos);
            }
        }
    });

    let on_select = Callback::new(move |value: V| {
        on_change.run(value);
        is_open.set(false);
        selected_pos.set(None);
        if let Some(button_ref) = button_ref.get_untracked() {
            button_ref.focus().expect("focus");
        }
    });

    let on_cancel = Callback::new(move |()| {
        selected_pos.set(None);
        if let Some(button_ref) = button_ref.get_untracked() {
            button_ref.focus().expect("focus");
            is_open.set(false);
        }
    });

    view! {
        <div node_ref=container_ref class="listbox-container">

            <button
                id=format!("widget-{}-listbox-button", widget_id)

                type="button"

                aria-haspopup="listbox"

                aria-expanded=move || is_open.get().to_string()

                aria-controls=move || {
                    if is_open.get() {
                        Some(format!("widget-{}-listbox-options", widget_id))
                    } else {
                        None
                    }
                }

                node_ref=button_ref

                class="listbox-button"

                on:click=move |_| {
                    is_open.set(!is_open.get());
                }
            >
                {definition.button.view.run()}
            </button>
            <div class="listbox-options-container">
                <Show when=move || is_open.get()>
                    <ul
                        id=format!("widget-{}-listbox-options", widget_id)

                        role="listbox"

                        class="listbox-options"

                        aria-orientation="vertical"

                        aria-labelledby=format!(
                            "widget-{}-listbox-button",
                            widget_id,
                        )

                        aria-activedescendant=move || {
                            if let Some(selected_pos) = selected_pos.get() {
                                Some(
                                    format!(
                                        "widget-{}-listbox-option-{}",
                                        widget_id,
                                        selected_pos,
                                    ),
                                )
                            } else {
                                None
                            }
                        }
                    >

                        {definition
                            .options
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
                                        if Some(pos) == selected_pos.get() {
                                            if let Some(node_ref) = node_ref.get() {
                                                node_ref.focus().expect("focus");
                                            }
                                        }
                                    });
                                    view! {
                                        <li
                                            id=format!("widget-{}-listbox-option-{}", widget_id, pos)

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
                                                            on_cancel.run(());
                                                        }
                                                    } else {
                                                        on_cancel.run(());
                                                    }
                                                }
                                            }

                                            on:click={
                                                let value = option.value.clone();
                                                move |_| { on_select.run(value.clone()) }
                                            }

                                            on:keydown={
                                                let value = option.value.clone();
                                                move |event| {
                                                    let key = event.key();
                                                    match key.as_str() {
                                                        "ArrowUp" => {
                                                            if let Some(pos) = selected_pos.get_untracked() {
                                                                if pos > 0 {
                                                                    selected_pos.set(Some(pos - 1));
                                                                }
                                                            }
                                                        }
                                                        "ArrowDown" => {
                                                            if let Some(pos) = selected_pos.get_untracked() {
                                                                if pos < length - 1 {
                                                                    selected_pos.set(Some(pos + 1));
                                                                }
                                                            }
                                                        }
                                                        " " | "Enter" => {
                                                            on_select.run(value.clone());
                                                            event.prevent_default();
                                                        }
                                                        "Tab" => {
                                                            event.prevent_default();
                                                        }
                                                        "Escape" => {
                                                            on_cancel.run(());
                                                        }
                                                        _ => {}
                                                    }
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
