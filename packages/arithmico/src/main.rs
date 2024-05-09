use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("absolute", "inset-0", "grid", "grid-rows-[auto_1fr]")}>
            <nav class="w-full flex justify-center items-center bg-blue-400">
                <h1 class="pr-4">{"Arithmico"}</h1>
                <ul class="flex">
                    <li class="p-2">{"Rechner"}</li>
                    <li class="p-2">{"Einstellungen"}</li>
                    <li class="p-2">{"Hilfe"}</li>
                    <li class="p-2">{"Über"}</li>
                </ul>
            </nav>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
