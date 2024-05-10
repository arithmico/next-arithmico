use yew::prelude::*;

use crate::components::{Navbar, Page};
mod components;

#[function_component]
fn App() -> Html {
    html! {
        <Page>
            <Navbar>
                <li class="p-2">{"Rechner"}</li>
                <li class="p-2">{"Einstellungen"}</li>
                <li class="p-2">{"Hilfe"}</li>
                <li class="p-2">{"Über"}</li>
            </Navbar>
        </Page>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
