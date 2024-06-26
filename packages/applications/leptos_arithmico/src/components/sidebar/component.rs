use leptos::*;

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    view! {
        <aside class="bg-white h-screen border-r border-neutral-300 px-2 flex flex-col">
            <h1 class="py-4 pr-10 text-2xl font-light">Arithmico</h1>
            {children()}
        </aside>
    }
}
