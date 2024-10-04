use leptos::*;
use leptos_router::*;

#[component]
pub fn NavigationItem(children: Children, to: String) -> impl IntoView {
    view! {
        <li class="flex flex-col">
            <A class="py-2 px-6 hover:bg-neutral-200 flex items-center rounded-md" href=to>
                {children()}
            </A>
        </li>
    }
}
