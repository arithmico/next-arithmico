use leptos::{html, logging::log, prelude::*};

use super::ListboxDefinition;

use crate::{class_names, widget_id::use_widget_id};

#[derive(Clone)]
struct ListboxContext {
    is_open: ReadSignal<bool>,
}

pub fn use_listbox_is_open() -> ReadSignal<bool> {
    log!("use_is_open");
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
    let close_lock = RwSignal::new(false);

    log!("provide context");
    provide_context(ListboxContext {
        is_open: is_open.split().0,
    });

    Effect::new(move |_| {
        log!("toggle: {}", is_open.get());
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
            if !close_lock.get() {
                is_open.set(false);
            }
            close_lock.set(false);
        }
    });

    view! {
        <div class=move || {
            class_names!(
                "flex",
                "relative",
                "flex-col",
                "ml-auto",
                "w-32",
            )
        }>
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

                class=move || {
                    class_names!(
                        "flex",
                        "py-1",
                        "px-2",
                        "text-left",
                        "rounded-sm",
                        "border",
                        "theme-light:bg-neutral-200",
                        "theme-dark:bg-neutral-700",
                        "border-neutral-300",
                        "theme-light:hover:bg-neutral-300",
                        "theme-dark:hover:bg-neutral-600",
                        definition.button.class.get(),
                    )
                }

                on:mousedown=move |_| {
                    close_lock.set(true);
                }

                on:mouseleave=move |_| {
                    close_lock.set(false);
                }

                on:mouseup=move |_| {
                    close_lock.set(false);
                }

                on:click=move |_| {
                    is_open.set(!is_open.get());
                }
            >
                {definition.button.view.run()}
            </button>
            <div class="flex relative flex-col">
                <Show when=move || is_open.get()>
                    <ul
                        id=format!("widget-{}-listbox-options", widget_id)

                        role="listbox"

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

                        class=move || {
                            class_names!(
                                "absolute",
                                "z-10",
                                "mt-1",
                                "w-full",
                                "border",
                                "theme-light:bg-neutral-200",
                                "theme-dark:bg-neutral-700",
                                "border-neutral-300",
                                "rounded-sm",
                                definition.options.class.get()
                            )
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
                                                close_lock.set(false);
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
                                                class_names!(
                                                    "flex",
                                                    "items-center",
                                                    "px-2",
                                                    "py-1",
                                                    "theme-light:hover:bg-neutral-300",
                                                    "theme-dark:hover:bg-neutral-600",
                                                    "rounded-sm",
                                                    "focus-visible:outline-2",
                                                    "outline-black",
                                                    if selected.get() {
                                                        "font-bold"
                                                    } else {
                                                        "font-normal"
                                                    },
                                                    option.class.get(),
                                                )
                                            }

                                            on:mousedown=move |_| {
                                                close_lock.set(true);
                                            }

                                            on:mouseleave=move |_| {
                                                close_lock.set(false);
                                            }

                                            on:mouseup=move |_| {
                                                close_lock.set(false);
                                            }

                                            on:focusout=move |_| {
                                                if !close_lock.get() {
                                                    on_cancel.run(());
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
                                                                    close_lock.set(true);
                                                                    selected_pos.set(Some(pos - 1));
                                                                }
                                                            }
                                                        }
                                                        "ArrowDown" => {
                                                            if let Some(pos) = selected_pos.get_untracked() {
                                                                if pos < length - 1 {
                                                                    close_lock.set(true);
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
