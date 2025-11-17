use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod about;
mod calculator;
mod definitions;
mod history;
mod not_found;
mod reference;
mod settings;

pub use about::AboutPage;
pub use calculator::CalculatorPage;
pub use definitions::DefinitionsPage;
pub use history::HistoryPage;
pub use not_found::NotFoundPage;
pub use reference::ReferencePage;
pub use settings::SettingsPage;

#[component]
pub fn ApplicationRouter() -> impl IntoView {
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
