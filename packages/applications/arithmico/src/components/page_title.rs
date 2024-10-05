use leptos::*;

#[component]
pub fn PageTitle(children: Children) -> impl IntoView {
    view! { <h1 class="py-4 pr-10 text-2xl font-medium">{children()}</h1> }
}
