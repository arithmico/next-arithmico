use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::pages::*;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFoundPage>
                <Route path=path!("/") view=CalculatorPage />
                <Route path=path!("/settings") view=SettingsPage />
                <Route path=path!("/reference") view=ReferencePage />
                <Route path=path!("/about") view=AboutPage />
                <Route path=path!("/history") view=HistoryPage />
                <Route path=path!("/definitions") view=DefinitionsPage />
            </Routes>
        </Router>
    }
}
