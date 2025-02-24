use crate::pages::*;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFoundPage>
                <Route path=path!("/") view=CalculatorPage />
                <Route path=path!("/settings") view=SettingsPage />
                <Route path=path!("/reference") view=ReferencePage />
                <Route path=path!("/about") view=AboutPage />
            </Routes>
        </Router>
    }
}
