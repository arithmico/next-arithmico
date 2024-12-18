use crate::pages::*;
use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub fn Router() -> impl IntoView {
    view! {
        <LeptosRouter>
            <Routes>
                <Route path="/" view=CalculatorPage />
                <Route path="/settings" view=SettingsPage />
                <Route path="/reference" view=ReferencePage />
                <Route path="/about" view=AboutPage />
            </Routes>
        </LeptosRouter>
    }
}
