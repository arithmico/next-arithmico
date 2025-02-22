use leptos::prelude::*;

#[component]
pub fn Card(
    children: Children,
    #[prop(into, default = Signal::derive(|| String::new()))] class: Signal<
        String,
    >,
) -> impl IntoView {
    view! { <div class=move || format!("card {}", class.get())>{children()}</div> }
}
