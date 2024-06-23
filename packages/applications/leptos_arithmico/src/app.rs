use leptos::*;
use leptos_router::*;

use crate::pages::calculator::CalculatorPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=CalculatorPage/>
            </Routes>
        </Router>
    }
}
