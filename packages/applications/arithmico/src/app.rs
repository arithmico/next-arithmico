use leptos::*;

use crate::router::Router;
use crate::state::StateProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <StateProvider>
            <Router/>
        </StateProvider>
    }
}
