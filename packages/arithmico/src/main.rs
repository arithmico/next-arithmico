use yew::prelude::*;

use crate::pages::CalculatorPage;
mod components;
mod pages;

#[function_component]
fn App() -> Html {
    html! {
        <CalculatorPage />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
