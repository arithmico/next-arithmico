use leptos::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="max-h-full absolute inset-0 bg-neutral-100 grid grid-cols-[minmax(10%,auto)_1fr] overflow-hidden">
            {children()}
        </div>
    }
}
