use leptos::{html, prelude::*};

use crate::form::menu::{menu_context::MenuContext, menu_item::MenuItem};

use super::MenuDefinition;

#[component]
pub fn MenuItems(
    definition: MenuDefinition,
    container_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let context = expect_context::<RwSignal<MenuContext>>();

    view! {
        <ul
            id=move || {
                format!("widget-{}-menu-items", context.get().widget_id())
            }

            aria-labelledby=move || {
                format!("widget-{}-menu-button", context.get().widget_id())
            }

            aria-activedescendant=move || {
                format!(
                    "widget-{}-menu-item-{}",
                    context.get().widget_id(),
                    context.get().get_position(),
                )
            }

            role="menu"

            class="menu-items"
        >
            {definition
                .items()
                .iter()
                .enumerate()
                .map(|(pos, item)| {
                    view! {
                        <MenuItem
                            item=item.clone()
                            pos=pos
                            container_ref=container_ref
                        />
                    }
                })
                .collect_view()}
        </ul>
    }
}
