use leptos::{html, prelude::*};
use web_sys::{wasm_bindgen::JsCast, Node};

use super::ListboxDefinition;

use crate::{
    control::focus_ring::{FocusRing, FocusRingOrientation},
    widget_id::use_widget_id,
};

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
    let button_ref = NodeRef::<html::Button>::new();
    let container_ref = NodeRef::<html::Div>::new();

    let focus_ring = RwSignal::new(FocusRing::new(
        definition
            .position_of(&value.get_untracked())
            .expect("position"),
        definition.len(),
        FocusRingOrientation::Vertical,
    ));

    provide_context(ListboxContext {
        is_open: is_open.split().0,
    });

    Effect::new({
        let definition = definition.clone();
        move |_| {
            let position = definition.position_of(&value.get());
            if let Some(position) = position {
                focus_ring
                    .update(|focus_ring| focus_ring.set_position(position));
            }
        }
    });

    let on_select = Callback::new(move |value: V| {
        on_change.run(value);
        is_open.set(false);
        if let Some(button_ref) = button_ref.get_untracked() {
            button_ref.focus().expect("focus");
        }
    });

    let on_cancel = Callback::new(move |()| {
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
                {definition.button.run()}
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
                            if is_open.get() {
                                Some(
                                    format!(
                                        "widget-{}-listbox-option-{}",
                                        widget_id,
                                        focus_ring.get().get_position(),
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
                                        let focus_ring = focus_ring.get();
                                        if pos == focus_ring.get_position() {
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
                                                    let mut new_focus_ring = focus_ring.get();
                                                    if new_focus_ring.handle_keydown(&key) {
                                                        focus_ring.set(new_focus_ring);
                                                    } else {
                                                        match key.as_str() {
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
