use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::router::{switch, Route};
mod components;
mod pages;
mod router;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
