use leptos::{html, logging::error, prelude::*};
use web_sys::{Node, wasm_bindgen::JsCast};

use crate::form::menu::menu_context::MenuContext;

use super::menu_definition::MenuItemDefinition;

#[component]
pub fn MenuItem(
    item: MenuItemDefinition,
    pos: usize,
    container_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let context = expect_context::<RwSignal<MenuContext>>();
    let node_ref = NodeRef::<html::Li>::new();

    Effect::new(move |_| {
        if context.get().get_position() != pos {
            return;
        }
        if let Some(node_ref) = node_ref.get()
            && let Err(err) = node_ref.focus()
        {
            error!("Failed to focus node! {:?}", err);
        };
    });

    view! {
        <li
            id=move || {
                format!(
                    "widget-{}-menu-item-{}",
                    context.get().widget_id(),
                    pos,
                )
            }

            role="menuitem"

            node_ref=node_ref

            class="menu-item"

            tabindex="-1"

            on:keydown={
                let action = item.action_callback();
                move |event| {
                    context
                        .update(|context| { context.on_keydown(event, action) });
                }
            }

            on:click=move |_| {
                context.update(|context| context.submit());
                item.action();
            }

            on:focusout=move |e| {
                let target = e.related_target();
                if let Some(container_ref) = container_ref.get_untracked() {
                    if let Some(target) = target {
                        let target = target.dyn_into::<Node>().ok();
                        if !container_ref.contains(target.as_ref()) {
                            context.update(|context| context.cancel());
                        }
                    } else {
                        context.update(|context| context.cancel());
                    }
                }
            }
        >
            {item.view()}
        </li>
    }
}
