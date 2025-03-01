use leptos::prelude::*;

#[component]
pub fn Page(
    children: Children,
    #[prop(default = None)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class=if let Some(class) = class {
            format!("page {}", class)
        } else {
            String::from("page")
        }>{children()}</div>
    }
}
