use leptos::prelude::*;

use crate::{app_shell::AppShell, pages::ApplicationRouter};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AppShell>
            <ApplicationRouter />
        </AppShell>
    }
}
