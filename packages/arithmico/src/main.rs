use std::rc::Rc;

use engine::load_host_api;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{
    app_context::AppContext,
    router::{switch, Route},
};
mod app_context;
mod components;
mod pages;
mod router;

#[function_component]
fn App() -> Html {
    let context = AppContext {
        host_api: Rc::new(load_host_api()),
    };

    html! {
        <ContextProvider<AppContext> context={context}>
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </ContextProvider<AppContext>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
