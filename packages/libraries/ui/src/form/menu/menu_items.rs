use leptos::{html, prelude::*};

use crate::form::menu::menu_item::MenuItem;

use super::MenuDefinition;

#[component]
pub fn MenuItems(
    definition: MenuDefinition,
    container_ref: NodeRef<html::Div>,
) -> impl IntoView {
    view! {
        <ul class="menu-items">
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
