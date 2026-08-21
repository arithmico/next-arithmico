use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod about;
mod calculator;
mod definitions;
mod font_license;
mod history;
mod license;
mod not_found;
mod reference;
mod reference_details;
mod settings;

pub use about::AboutPage;
pub use calculator::CalculatorPage;
pub use definitions::DefinitionsPage;
pub use font_license::FontLicensePage;
pub use history::HistoryPage;
pub use not_found::NotFoundPage;
pub use reference::ReferencePage;
pub use settings::SettingsPage;

use crate::pages::license::LicensePage;
use crate::pages::reference_details::ReferenceDetailsPage;

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
                <Route
                    path=path!("/reference/:endpoint_name")
                    view=ReferenceDetailsPage
                />
                <Route path=path!("/about/license") view=LicensePage />
                <Route path=path!("/about/font_license") view=FontLicensePage />
            </Routes>
        </Router>
    }
}
