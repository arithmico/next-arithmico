use leptos::*;

#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! {
        <div class="grid overflow-hidden absolute inset-0 max-h-full theme-light:bg-neutral-100 theme-dark:bg-black grid-cols-[minmax(10%,auto)_1fr]">
            {children()}
        </div>
    }
}
