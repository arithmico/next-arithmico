use leptos::{html, prelude::*};

use crate::form::menu::{menu_context::MenuContext, menu_items::MenuItems};

use super::MenuDefinition;

#[component]
pub fn Menu(definition: MenuDefinition) -> impl IntoView {
    let container_ref = NodeRef::<html::Div>::new();
    let button_ref = NodeRef::<html::Button>::new();
    let focus_button = Callback::new(move |_| {
        if let Some(button) = button_ref.get() {
            button.focus().expect("focus");
        }
    });
    let context = RwSignal::new(MenuContext::new(
        &definition,
        focus_button,
        focus_button,
    ));
    provide_context(context);

    view! {
        <div class="menu-container" node_ref=container_ref>
            <button
                id=move || {
                    format!("widget-{}-menu-button", context.get().widget_id())
                }

                aria-haspopup="menu"

                aria-expanded=move || context.get().is_open().to_string()

                aria-controls=move || {
                    format!("widget-{}-menu-items", context.get().widget_id())
                }

                node_ref=button_ref

                class="menu-button"

                on:click=move |_| {
                    context.update(|context| context.toggle());
                }
            >
                {definition.button_view().run()}
            </button>
            <div class="menu-items-container">
                <Show when=move || context.get().is_open()>
                    <MenuItems
                        definition=definition.clone()
                        container_ref=container_ref
                    />
                </Show>
            </div>
        </div>
    }
}
