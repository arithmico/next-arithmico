use leptos::prelude::*;

#[component]
pub fn NavigationItem(children: Children, to: String) -> impl IntoView {
    view! {
        <li class="navigation-item">
            <a href=to>{children()}</a>
        </li>
    }
}
