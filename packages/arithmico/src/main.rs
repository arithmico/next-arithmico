use yew::prelude::*;

use crate::components::page::Page;
mod components;

#[function_component]
fn App() -> Html {
    html! {
        <Page>
            <nav class="w-full flex justify-center items-center bg-blue-400">
                <h1 class="pr-4">{"Arithmico"}</h1>
                <ul class="flex">
                    <li class="p-2">{"Rechner"}</li>
                    <li class="p-2">{"Einstellungen"}</li>
                    <li class="p-2">{"Hilfe"}</li>
                    <li class="p-2">{"Über"}</li>
                </ul>
            </nav>
        </Page>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
