use crate::pages::*;
use yew::{html, Html, Properties};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Calculator,

    #[at("/settings")]
    Settings,

    #[at("/help")]
    Help,

    #[at("/about")]
    About,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Calculator => html!(<CalculatorPage />),
        _ => html!( <p>{"Not Found"}</p>),
    }
}
