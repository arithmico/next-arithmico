use ev::KeyboardEvent;
use leptos::prelude::*;

use crate::form::listbox::{
    ListboxContext, ListboxDispatch, ListboxDispatchAction,
};

use super::ListboxOptionDefinition;

#[component]
pub(super) fn InternalOption<T>(
    option: ListboxOptionDefinition<T>,
) -> impl IntoView
where
    T: PartialEq + Clone + 'static,
{
    let context = use_context::<ReadSignal<ListboxContext<T>>>()
        .expect("listbox context");
    let dispatch = use_context::<ListboxDispatch<T>>()
        .expect("listbox dispatch")
        .0;

    let value = option.value.clone();

    let is_current = {
        let value = value.clone();
        move || context.with(|context| context.current.eq(&value))
    };

    let is_focused = {
        let value = value.clone();
        move || context.with(|context| context.focused.eq(&value))
    };

    let active_class = option
        .class
        .as_ref()
        .map(|class| class.get_class(true))
        .unwrap_or(String::new());

    let class = option
        .class
        .as_ref()
        .map(|class| class.get_class(false))
        .unwrap_or(String::new());

    let node_ref = create_node_ref::<html::Li>();
    {
        let is_focused = is_focused.clone();
        create_effect(move |_| {
            let is_open = context.get().is_open;
            let is_selected = is_focused();
            if is_open && is_selected {
                if let Some(node) = node_ref.get() {
                    node.focus().expect("focus");
                }
            }
        });
    }

    view! {
        <li
            node_ref=node_ref
            aria-selected={
                let is_current = is_current.clone();
                move || is_current().to_string()
            }
            tabindex="-1"
            class=move || {
                format!("{}", if is_current() { &active_class } else { &class })
            }
            on:click={
                let value = value.clone();
                move |_| {
                    dispatch.call(ListboxDispatchAction::Change(value.clone()))
                }
            }

            on:mouseup=move |_| {
                dispatch.call(ListboxDispatchAction::BlurLock(false));
            }

            on:mousedown=move |_| {
                dispatch.call(ListboxDispatchAction::BlurLock(true));
            }

            on:mouseleave=move |_| {
                dispatch.call(ListboxDispatchAction::BlurLock(false));
            }

            on:focusout=move |_| {
                dispatch.call(ListboxDispatchAction::BlurLock(false));
            }

            on:blur=move |_| {
                dispatch.call(ListboxDispatchAction::Blur);
            }

            on:keydown={
                let value = value.clone();
                move |event: KeyboardEvent| {
                    let key = event.key();
                    if key == "ArrowUp" {
                        dispatch.call(ListboxDispatchAction::SelectPrevious);
                    } else if key == "ArrowDown" {
                        dispatch.call(ListboxDispatchAction::SelectNext);
                    } else if key == " " || key == "Enter" {
                        dispatch.call(ListboxDispatchAction::Change(value.clone()));
                        event.prevent_default();
                    } else if key == "Tab" {
                        event.prevent_default();
                    } else if key == "Escape" {
                        dispatch.call(ListboxDispatchAction::Close(true));
                        event.prevent_default();
                    }
                }
            }
        >

            {option.children.clone()}
        </li>
    }
}
