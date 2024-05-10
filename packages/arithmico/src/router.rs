use crate::pages::*;
use yew::{html, Html};
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

    #[not_found]
    #[at("/not-found")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Calculator => html!(<CalculatorPage />),
        Route::Settings => html!(<SettingsPage />),
        Route::Help => html!(<HelpPage />),
        Route::About => html!(<AboutPage />),
        Route::NotFound => html!(<p>{"Not Found"}</p>),
    }
}
